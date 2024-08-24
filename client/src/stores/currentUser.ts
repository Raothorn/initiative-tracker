import $socket from '@/socket'
import { defineStore } from 'pinia'
import { computed, ref, type Ref } from 'vue'
import useGameState from './gameState'

export const useUser = defineStore('user', () => {
  // Data
  const playerName: Ref<string | null> = ref(null)
  const playerGuid: Ref<string> = ref(crypto.randomUUID().toString())


  // Getters
  const isLoggedIn = computed(() => {
    const gameState = useGameState()
    for (const player of gameState.players) {
      if (player.guid == playerGuid.value) {
        return true
      }
    }
    return false
  })

  // Mutators
  function login(_username: string, _initativeBonus: number) {
    playerName.value = _username

    $socket.sendAction('loginAction', {
      playerName: _username,
      playerGuid: playerGuid.value,
      playerInitiativeBonus: 0
    })
  }

  return { playerName, playerGuid, login, isLoggedIn }
})

export default useUser
