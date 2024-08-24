use serde::Serialize;

use super::{combatant::Combatant, Update};


#[derive(Clone)]
pub struct Encounter {
    combatants: Vec<Combatant>,
    turn_number: u32
}

impl Encounter {
    pub fn advance_turn(&self) -> Update<Self> {
        let mut gs = self.clone();
        gs.turn_number = (self.turn_number + 1) % self.combatants.len() as u32; 
        return Ok(gs);
    }

    fn get_current_turn_id(&self) -> u32 {
        let combatants = &self.get_sorted_combatants();
        combatants[self.turn_number as usize].id
    }

    fn get_sorted_combatants(&self) -> Vec<Combatant> {
        let mut combatants = self.combatants.clone();
        combatants.sort_by(|a, b|  { a.initiative().cmp(&b.initiative()) } );
        
        combatants
    }

    fn encode(&self) -> SerialEncounter {
        return SerialEncounter {
            combatants: self.get_sorted_combatants(),
            current_turn_id: self.get_current_turn_id(),
        }
    }
}

impl Serialize for Encounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.encode().serialize(serializer)
    }
}

#[derive(Serialize, Clone)]
pub struct SerialEncounter {
    combatants: Vec<Combatant>,

    #[serde(rename = "currentTurnId")]
    current_turn_id: u32
}

