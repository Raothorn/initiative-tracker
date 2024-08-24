import type { Encounter } from './Encounter'
import type { GamePhase } from './GamePhase'

export type GameState = {
    gamephase: GamePhase,
    players: Player[] 
}

// TODO refactor to file
type Player = {
    name: string,
    guid: string,
    initiativeBonus: number
}
