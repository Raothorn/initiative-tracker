// Utilities
import { defineStore } from 'pinia'
import { computed, ref, type Ref } from 'vue'
import type { GameState, Encounter, Combatant } from '../models'
import $socket from '@/socket'

export const useGameState = defineStore('gameState', () => {
  // gameState is null until getting the initial state from the server
  const gameState: Ref<GameState | null > = ref(null)

  // Setup gamestate updates via websocket
  $socket.$onStateUpdate((newState:GameState) => {
      console.log("updating state", newState)
      gameState.value = newState
  })

  const encounter = computed(() => {
    if (gameState.value && "EncounterPhase" in gameState.value.gamephase) {
      return gameState.value.gamephase.EncounterPhase.encounter
    }
    else {
      return null;
    }
  })

  const players = computed(() => {
    if (gameState.value) {
      return gameState.value.players
    }    
    else {
      return []
    }
  })

  const hasInitialized = computed(() => {
    return gameState.value != null;
  })

  // We hide the actual gamestate from the views so we can encapsulate some important stuff
  return { hasInitialized, players, encounter }
})

export default useGameState

export const useClientState = defineStore('clientstate', () => {
  return {}
})