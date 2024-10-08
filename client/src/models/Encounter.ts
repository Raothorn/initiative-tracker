import type { Combatant } from './Combatant'

export type Encounter = {
  combatants: Combatant[]
  currentTurnId: string
  nextTurnId: string
}
