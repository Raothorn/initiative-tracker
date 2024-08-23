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
    pub fn new(players: &Vec<&str>) -> Self {
        GameState {
            gamephase: GamePhase::SetupPhase,
            players: Vec::new()
        }
    }

    pub fn init() -> Self {
        let players = vec!["Gob 1", "Gob 2", "Gob 3"];
        GameState::new(&players)
    }
    
    pub fn set_phase(self, gamephase: GamePhase) -> Update<Self> {
        Ok(GameState {
            gamephase,
            ..self
        })
    }
}


type Player = String;
// #[derive(Serialize)]
// pub struct SerialGameState {
//     combatants: Vec<Combatant>,
//     #[serde(rename = "currentTurnId")]
//     current_turn_id: u32,
// }

type Update<T> = Result<T, String>;
