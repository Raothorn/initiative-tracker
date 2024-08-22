<template>
  <div class="alert text-center" :class="{ 'alert-primary': isMyTurn, 'alert-secondary': !isMyTurn}" role="alert">
    <template v-if="isMyTurn">
      Your turn! {{ isMyTurnNext }} {{ gameStateStore.nextTurnId }} {{ gameStateStore.gameState.currentTurnId }}
    </template>
    <template v-else-if="isMyTurnNext">
      Up next {{ isMyTurnNext }} {{ gameStateStore.nextTurnId }} {{ gameStateStore.gameState.currentTurnId }}
    </template>
    <template v-else>
      Waiting... {{ isMyTurnNext }} {{ gameStateStore.nextTurnId }} {{ gameStateStore.gameState.currentTurnId }}
    </template>
  </div>
</template>

<script setup lang="ts">
import useUser from '@/stores/currentUser';
import useGameState from '@/stores/gameState'
import { computed } from 'vue'

const gameStateStore = useGameState()
const userStore = useUser()

const isMyTurn = computed(() => {
  return gameStateStore.gameState.currentTurnId == userStore.userId
})

const isMyTurnNext = computed(() => {
  return gameStateStore.nextTurnId == userStore.userId
})

</script>
