// Utilities
import { defineStore } from 'pinia'
import { ref, type Ref } from 'vue'
import type { Encounter, Combatant } from '../models'

export type GameState = Encounter

export const useGameState = defineStore('gamestate', () => {
  const gameState: Ref<GameState> = ref(mockGameState())
  return { gameState }
})

export default useGameState

export const useClientState = defineStore('clientstate', () => {
  return {}
})

function mockGameState(): GameState {
  let combatants: Combatant[] = []
  for (let i = 0; i < 5; i++) {
    combatants[i] = mockCombatant(i)
  }
  return { combatants: combatants }
}

function mockCombatant(initiative: number): Combatant {
  return { name: 'Goblin', initMod: 1, initRoll: initiative }
}
