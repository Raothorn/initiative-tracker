use serde::{Deserialize, Serialize};
use std::fmt;

use crate::gamestate::{gamephase::GamePhase, GameState, Player};

use super::{Action, Update};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginAction {
    player_name: String,
    player_guid: String,
    player_initiative_bonus: i32,
}

#[typetag::serde(name = "loginAction")]
impl Action for LoginAction {
    fn execute(&self, gs: &GameState) -> Update {
        let gs = gs.clone();

        if let GamePhase::SetupPhase {} = gs.gamephase {
            let player = Player {
                name: self.player_name.clone(),
                guid: self.player_guid.clone(),
                initiative_bonus: self.player_initiative_bonus,
            };

            Ok(gs)
                .and_then(|g| g.add_player(player))
        } else {
            Err("Cannot join once encounter phase has started.".to_owned())
        }
    }
}

impl fmt::Display for LoginAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Add player action")
    }
}
