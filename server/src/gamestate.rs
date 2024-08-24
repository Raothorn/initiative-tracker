pub mod gamephase;
mod combatant;
pub mod encounter;


use serde::Serialize;
use gamephase::GamePhase;

#[derive(Serialize, Clone)]
pub struct GameState {
    pub gamephase: GamePhase,
    players: Vec<Player>
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            gamephase: GamePhase::SetupPhase{},
            players: Vec::new()
        }
    }

    pub fn add_player(self, player: Player) -> Update<Self> {
        let mut gs = self.clone();
        gs.players.push(player);
        Ok(gs)
    }

    pub fn remove_player(&mut self, guid: &str) {
        println!("GUID: {}", guid);
        println!("PLAYER COUNT:{}", self.players.len() );
        println!("PLAYERS: {:?}", self.players);
        self.players.retain(|p| p.guid.as_str() == guid);
        println!("PLAYER COUNT:{}", self.players.len() );
        println!("PLAYERS2: {:?}", self.players);
    }

   
    pub fn set_phase(self, gamephase: GamePhase) -> Update<Self> {
        Ok(GameState {
            gamephase,
            ..self
        })
    }
}


#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub guid: String,
    pub initiative_bonus: i32
}


type Update<T> = Result<T, String>;
