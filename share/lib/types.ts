export type Combatant = {
    name: String,
    max_hitpoints: number,
    current_hitpoints: number,
    initiative: number
}

export type Encounter = {
    combatants: Combatant[]
}

export type GameState = Encounter
