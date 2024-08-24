import { type Encounter } from "./Encounter"

export type GamePhase = 
    | { SetupPhase: {} }
    | { EncounterPhase: { encounter: Encounter } }