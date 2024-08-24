use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use serde_json::{json, Value};
use websocket::{
    sync::{Server, Writer}, Message, OwnedMessage
};

use crate::{
    action::{self, get_action, Action},
    gamestate::GameState,
};

// type Client = Writer<TcpStream>;
struct Client {
    writer: Writer<TcpStream>,
    guid: Option<String>
}

impl Client {
    fn send_message(&mut self, msg: &str) -> Result<(), websocket::WebSocketError> {
        self.writer.send_message(&Message::text(msg))
    }
}

struct GameManager {
    state: GameState,
}

impl GameManager {
    fn execute_action(&mut self, action: &dyn Action) -> Option<String> {
        let res = action.execute(&self.state);

        match res {
            Ok(gs) => {
                self.state = gs;
                None
            }
            Err(err) => Some(err.to_owned()),
        }
    }

}

struct ServerState {
    manager: Mutex<GameManager>,
    clients: Mutex<HashMap<String, Client>>,
}

impl ServerState {
    fn add_client(&self, addr: &str, client: Client) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(addr.to_owned(), client);
    }

    fn remove_client(&self, addr: &str) {
        // remove from local clients list    
        let mut clients = self.clients.lock().unwrap();
        let guid = clients.get(addr).and_then(|c| c.guid.clone());

        clients.remove(addr);

        if let Some(guid) = guid {
            let mut manager = self.manager.lock().unwrap();
            manager.state.remove_player(&guid);

        }
    }

    fn client_set_guid(&self, addr: &str, guid: &str) {
        let mut clients = self.clients.lock().unwrap();
        let client = clients.get_mut(addr);
    
        if let Some(client) = client {
            if client.guid.is_some() {
                println!("Client GUID already set")
            }
            else {
                client.guid = Some(guid.to_owned());
                println!("Client logged in with GUID {:?}", client.guid);
            }
        }
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
            let _ = client.send_message(msg);
        }
    }

    fn handle_action_message(&self, addr: &str, msg: &str) {
        let mut manager = self.manager.lock().unwrap();
        let action = get_action(msg);

        if action.to_string() == "" {
            println!("Empty action message: {}", msg);
        } else {
            println!("{}", action);
        }

        let result = manager.execute_action(action.as_ref());
        match result {
            Some(err) => {
                println!("Error executing action: {}", err);
            }
            None => {
                drop(manager);
                self.broadcast_gamestate();
            }
        }
    }

    fn handle_message(&self, addr: &str, msg: &str) {
        // println!("received message: {}", msg);
        let msg: Value = serde_json::from_str(msg).unwrap();

        if let Value::Object(obj) = msg {
            let msg_type = obj.get("msgType");
            let msg_data = obj.get("msgData");

            if let (Some(Value::String(msg_type)), Some(msg_data)) = (msg_type, msg_data) {
                match msg_type.as_str() {
                    "action" => self.handle_action_message(addr, &msg_data.to_string()),
                    "setGuid" => self.client_set_guid(addr, &msg_data.to_string()),
                    _ => (),
                }
            }
        }
    }
}

pub fn run_server() {
    let state = Arc::new(ServerState {
        manager: Mutex::new(GameManager {
            state: GameState::new(),
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

            state.add_client(&client_addr, Client { writer: sender, guid: None });

            state.broadcast_gamestate();

            for message in reciever.incoming_messages() {
                match message {
                    Ok(OwnedMessage::Text(txt)) => {
                        state.handle_message(&client_addr, &txt);
                    }
                    Ok(OwnedMessage::Close(_)) => {
                        state.remove_client(&client_addr);
                        println!("Client disconnected");
                        break;
                    }
                    _ => {}
                }
            }
        });
    }
}
