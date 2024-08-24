use serde::{Deserialize, Serialize};
use std::fmt;

use crate::gamestate::{encounter::Encounter, gamephase::GamePhase, GameState, Player};

use super::{Action, Update};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct StartEncounterAction {
    selected_player_guids: Vec<String>, 
}

#[typetag::serde(name = "startEncounterAction")]
impl Action for StartEncounterAction {
    fn execute(&self, gs: &GameState) -> Update {
        let gs = gs.clone();

        if let GamePhase::SetupPhase {} = gs.gamephase {
            let selected_players: Vec<Player> = 
                gs.players.clone().into_iter()
                    .filter(|p| self.selected_player_guids.contains(&p.guid))
                    .collect();

            if selected_players.len() > 0 {
                let encounter = Encounter::from_players(selected_players);
                let encounter_phase = GamePhase::EncounterPhase { encounter };
                gs.set_phase(encounter_phase)
            }
            else {
                Err ("At least one combatant must be selected".to_owned())
            }

        } else {
            Err("Encounter already started.".to_owned())
        }
    }
}

impl fmt::Display for StartEncounterAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Starting encounter with {} players", self.selected_player_guids.len())
    }
}
