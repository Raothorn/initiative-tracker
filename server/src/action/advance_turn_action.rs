use std::fmt;

use serde::{Deserialize, Serialize};

use crate::gamestate::GameState;

use super::{Action, Update};

#[derive(Deserialize, Serialize)]
struct AdvanceTurnAction;

#[typetag::serde(name = "advanceTurnAction")]
impl Action for AdvanceTurnAction {
    fn execute(&self, gs: &GameState) -> Update {
        gs.advance_turn()
    }
}

impl fmt::Display for AdvanceTurnAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Advance turn action")
    }
}
