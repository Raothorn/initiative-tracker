use serde::Serialize;

#[derive(Serialize)]
pub struct GameState {
    combatants: Vec<Combatant>,
    #[serde(rename = "currentTurnId")]
    current_turn_id: i32
}

impl GameState {
    pub fn new(players: &Vec<&str>) -> Self {
        let combatants = players
            .iter()
            .enumerate()
            .map(|(ix, name)| Combatant {
                id: ix as i32,
                name: name.to_string(),
                user: "Yo".to_owned(),
                init_roll: 2,
                init_mod: 1,
            })
            .collect();

        GameState {
            combatants,
            current_turn_id: 0
        }
    }

    pub fn init() -> Self {
        let players = vec!["Gob 1", "Gob 2", "Gob 3"];
        GameState::new(&players)
    }

}

#[derive(Serialize)]
pub struct Combatant {
    id: i32,
    name: String,
    user: String,
    #[serde(rename = "initRoll")]
    init_roll: i32,
    #[serde(rename = "initMod")]
    init_mod: i32,
}

