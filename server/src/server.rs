// use serde_json::{json, Value};

use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use serde_json::json;
use websocket::{
    sync::{Server, Writer},
    Message, OwnedMessage,
};

use crate::gamestate::GameState;


type Sender = Writer<TcpStream>;

struct GameManager {
    state: GameState,
}

struct ServerState {
    manager: Mutex<GameManager>,
    clients: Mutex<HashMap<String, Sender>>,
}

impl ServerState {
    fn add_client(&self, addr: &str, client: Sender) {
        let mut clients = self.clients.lock().unwrap();

        clients.insert(addr.to_owned(), client);
    }

    fn broadcast_gamestate(&self) {
        let manager = self.manager.lock().unwrap();

        let message = json!({
            "msgType": "update",
            "msgData": manager.state,
        });

        self.broadcast(&message.to_string())
    }

    fn broadcast(&self, msg: &str) {
        let mut clients = self.clients.lock().unwrap();

        for client in clients.values_mut() {
            let _ = client.send_message(&Message::text(msg));
        }
    }


    fn handle_message(&self, addr: &str, msg: &str) {
        println!("received message: {}", msg);
    }
}

pub fn run_server() {
    let state = Arc::new(ServerState {
        manager: Mutex::new(GameManager {
            state: GameState::init(),
        }),
        clients: Mutex::new(HashMap::new()),
    });
    let wsserver = Server::bind("localhost:9001").unwrap();

    for connection in wsserver.filter_map(Result::ok) {
        let state = Arc::clone(&state);
        // let clients = Arc::clone(&clients);
        let _ = thread::spawn(move || {
            let client = connection.accept().unwrap();
            let client_addr = client.peer_addr().unwrap().to_string();

            println!("{}", client_addr);

            // On initial connection

            let (mut reciever, sender) = client.split().unwrap();
            state.add_client(&client_addr, sender);

            state.broadcast_gamestate();

            for message in reciever.incoming_messages() {
                match message {
                    Ok(OwnedMessage::Text(txt)) => {
                        state.handle_message(&client_addr, &txt);
                    }
                    Ok(OwnedMessage::Close(_)) => {
                        println!("closing");
                        // Comment this line to prevent restart on reload
                        // state.reset();
                        break;
                    }
                    _ => {}
                }
            }
        });
    }
}
