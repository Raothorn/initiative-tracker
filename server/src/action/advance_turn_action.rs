use std::fmt;

use serde::{Deserialize, Serialize};

use crate::gamestate::{gamephase::GamePhase, GameState};

use super::{Action, Update};

#[derive(Deserialize, Serialize)]
struct AdvanceTurnAction;

#[typetag::serde(name = "advanceTurnAction")]
impl Action for AdvanceTurnAction {
    fn execute(&self, gs: &GameState) -> Update {
        let gs = gs.clone();

        if let GamePhase::EncounterPhase{encounter: ref encounter} = gs.gamephase {
            encounter
                .advance_turn()
                .and_then(|e| { gs.set_phase(GamePhase::EncounterPhase {encounter: e} ) })
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
