export type Combatant = {
    name: String,
    maxHitpoints: number,
    currentHitpoints: number,
    initiative: number
}

export type Encounter = {
    combatants: Combatant[]
}

export type GameState = Encounter
