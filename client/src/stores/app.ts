// Utilities
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { GameState, Combatant } from 'share'

export const useGameState = defineStore('gamestate', () => {
    const gameState: Ref<GameState> = ref(mockGameState());
    return { gameState }
})

export default useGameState;

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

function mockCombatant(initiative: number) : Combatant {
  return { name: "Goblin", maxHitpoints: 10, currentHitpoints: 10, initiative: initiative }
}

