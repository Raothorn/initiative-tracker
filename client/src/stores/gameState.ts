// Utilities
import { defineStore } from 'pinia'
import { computed, ref, type Ref } from 'vue'
import type { GameState, Encounter, Combatant } from '../models'

export const useGameState = defineStore('gameState', () => {
  const gameState: Ref<GameState> = ref(mockGameState())

  const sortedCombatants = computed(() => {
    let combatants = gameState.value.combatants
    combatants.sort((a, b) => (a.initRoll + a.initMod) - (b.initRoll + b.initMod))
    return combatants
  });

  const nextTurnId = computed(() => {
    let currentTurnIndex = sortedCombatants.value.findIndex(c => {c.id == gameState.value.currentTurnId})
    let numCombatants = gameState.value.combatants.length
    let nextTurnIndex = (currentTurnIndex+1)%numCombatants
    return sortedCombatants.value[nextTurnIndex].id
  });

  return { gameState, sortedCombatants, nextTurnId }
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
  let encounter: Encounter = {
    combatants: combatants,
    currentTurnId: 0,
    isActive: true,
    currentTurnCount: 0
  }
  return encounter
}

function mockCombatant(combatantId: number): Combatant {
  return { id: combatantId, name: 'Goblin', user: 'Your name', initMod: 1, initRoll: 10 }
}

function addCombatant() {

}

function advanceTurn() {
  

}

function rewindTurn() {

}

function startEncounter() {

}

function endEncounter() {

}