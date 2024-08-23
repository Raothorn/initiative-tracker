use std::fmt;

use serde::{Deserialize, Serialize};

use crate::gamestate::{gamephase::GamePhase, GameState};
use crate::gamestate::encounter::{Encounter, SerialEncounter};
use crate::util::encoded;

use super::{Action, Update};

#[derive(Deserialize, Serialize)]
struct AdvanceTurnAction;

#[typetag::serde(name = "advanceTurnAction")]
impl Action for AdvanceTurnAction {
    fn execute(&self, gs: &GameState) -> Update {
        let gs = gs.clone();

        if let GamePhase::EncounterPhase(ref encounter) = gs.gamephase {
            let advance_turn = encounter.raw().advance_turn();

            advance_turn
                .map(encoded::<Encounter, SerialEncounter> )
                .and_then(|e| { gs.set_phase(GamePhase::EncounterPhase(e))})
        }
        else {
            Err("Encounter phase not started".to_owned())
        }
    }
}

impl fmt::Display for AdvanceTurnAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Advance turn action")
    }
}
