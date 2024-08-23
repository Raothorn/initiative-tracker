use serde::Serialize;

use crate::util::Encoding;

use super::encounter::{Encounter, EncounterS, SerialEncounter};

#[derive(Serialize, Clone)]
pub enum GamePhase {
    SetupPhase,
    EncounterPhase(EncounterS)
}

