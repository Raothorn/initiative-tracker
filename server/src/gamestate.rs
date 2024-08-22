use serde::Serialize;

#[derive(Clone)]
pub struct GameState {
    combatants: Vec<Combatant>,
    current_turn_id: u32,
    turn_number: u32
}

impl GameState {
    pub fn new(players: &Vec<&str>) -> Self {
        let initiatives = vec![3, 12, 5];
        let combatants: Vec<Combatant> = players
            .iter()
            .enumerate()
            .map(|(ix, name)| Combatant {
                id: ix as u32,
                name: name.to_string(),
                user: "Yo".to_owned(),
                init_roll: initiatives[ix],
                init_mod: 1,
            })
            .collect();

        GameState {
            combatants: combatants.clone(),
            current_turn_id: combatants[0].id,
            turn_number: 0
        }
    }

    pub fn init() -> Self {
        let players = vec!["Gob 1", "Gob 2", "Gob 3"];
        GameState::new(&players)
    }

    pub fn serialize(&self) -> SerialGameState {
        SerialGameState {
            combatants: self.combatants.clone(),
            current_turn_id: self.get_current_turn_id(),
        }
    }

    pub fn advance_turn(&self) -> Result<Self, String> {
        let mut gs = self.clone();
        gs.turn_number = (self.turn_number + 1) % self.combatants.len() as u32; 
        gs.current_turn_id = self.get_current_turn_id();

        return Ok(gs);
    }

    fn sorted_combatants(&self) -> Vec<Combatant> {
        let mut combatants = self.combatants.clone();
        combatants.sort_by(|a, b|  { a.initiative().cmp(&b.initiative()) } );

        combatants
    }

    fn get_current_turn_id(&self) -> u32 {
        let combatants = &self.sorted_combatants();
        combatants[self.turn_number as usize].id
    }
}

// Serialization
#[derive(Serialize)]
pub struct SerialGameState {
    combatants: Vec<Combatant>,
    #[serde(rename = "currentTurnId")]
    current_turn_id: u32,
}

#[derive(Serialize, Clone)]
pub struct Combatant {
    id: u32,
    name: String,
    user: String,
    #[serde(rename = "initRoll")]
    init_roll: i32,
    #[serde(rename = "initMod")]
    init_mod: i32,
}

impl Combatant {
    fn initiative(&self) -> i32 {
        self.init_mod + self.init_roll
    }
}
