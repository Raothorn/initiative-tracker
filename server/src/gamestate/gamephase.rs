use serde::Serialize;

use super::encounter::Encounter;

#[derive(Serialize, Clone)]
pub enum GamePhase {
    SetupPhase {},
    EncounterPhase { encounter: Encounter }
}

#[derive(Serialize, Clone)]
pub struct SetupPhaseData {}

// Has to be a struct for serialization reasons ;(
// #[derive(Serialize, Clone)]
// pub struct EncounterPhaseData {
//     pub encounter: EncounterS
// }
