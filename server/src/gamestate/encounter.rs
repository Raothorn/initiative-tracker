use serde::Serialize;

use super::{combatant::Combatant, Player, Update};
#[derive(Clone)]
pub struct Encounter {
    combatants: Vec<Combatant>,
    turn_number: u32,
}

impl Encounter {
    pub fn from_players(players: Vec<Player>) -> Self {
        let combatants = players.iter().map(Combatant::from_player).collect();
        Encounter {
            combatants,
            turn_number: 0,
        }
    }

    pub fn advance_turn(&self) -> Update<Self> {
        let mut gs = self.clone();
        gs.turn_number = (self.turn_number + 1) % self.combatants.len() as u32;
        return Ok(gs);
    }

    fn get_current_turn_id(&self) -> Option<String> {
        let combatants = &self.get_sorted_combatants();
        combatants
            .get(self.turn_number as usize)
            .map(|c| c.id.clone())
    }

    fn get_next_turn_id(&self) -> Option<String> {
        let combatants = &self.get_sorted_combatants();
        let next_turn = (self.turn_number + 1) % self.combatants.len() as u32;
        combatants
            .get(next_turn as usize)
            .map(|c| c.id.clone())
    }

    fn get_sorted_combatants(&self) -> Vec<Combatant> {
        let mut combatants = self.combatants.clone();
        combatants.sort_by(|a, b| a.initiative().cmp(&b.initiative()));

        combatants
    }

    fn encode(&self) -> SerialEncounter {
        return SerialEncounter {
            combatants: self.get_sorted_combatants(),
            current_turn_id: self.get_current_turn_id().unwrap_or("UNDEFINED".to_owned()),
            next_turn_id: self.get_next_turn_id().unwrap_or("UNDEFINED".to_owned()),
        };
    }
}

impl Serialize for Encounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.encode().serialize(serializer)
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerialEncounter {
    combatants: Vec<Combatant>,
    current_turn_id: String,
    next_turn_id: String,
}
