use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::gamestate::GameState;

mod start_encounter_action;
mod login_action;
mod advance_turn_action;

#[typetag::serde(tag = "actionType", content = "actionData")]
pub trait Action: fmt::Display {
    fn execute(&self, state: &GameState) -> Result<GameState, String> {
        Ok(state.clone())
    }
}

pub fn get_action(action_msg_str: &str) -> Box<dyn Action> {
    let msg: Value = serde_json::from_str(action_msg_str).unwrap();

    let result = serde_json::from_value::<Box<dyn Action>>(msg);
    match result {
        Ok(action) => action,

        // We want to panic if an action isn't being parsed
        Err(e) => {
            println!("{}", e);
            Box::new(NoAction {})
         },
    }
}

// BASIC ACTIONS

#[derive(Deserialize, Serialize)]
struct NoAction;

#[typetag::serde(name = "noAction")]
impl Action for NoAction {
    fn execute(&self, gs: &GameState) -> Update {
        Ok(gs.to_owned())
    } 
}

impl Display for NoAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No action")
    }
}

pub type Update = Result<GameState, String>;
