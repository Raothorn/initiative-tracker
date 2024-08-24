<template>
  <div v-if="gameState.encounter" class="alert text-center" :class="{ 'alert-primary': isMyTurn, 'alert-secondary': !isMyTurn}" role="alert">
    <template v-if="isMyTurn">
      Your turn! 
    </template>
    <template v-else-if="isMyTurnNext">
      Up next
    </template>
    <template v-else>
      Waiting...
    </template>
  </div>
</template>

<script setup lang="ts">
import useUser from '@/stores/currentUser';
import useGameState from '@/stores/gameState'
import { computed } from 'vue'

const gameState = useGameState()
const user = useUser()

const isMyTurn = computed(() => {
  let currentTurnId = gameState.encounter?.currentTurnId
  if (currentTurnId != null) {
    return currentTurnId == user.userId
  }
  else {
    return false;
  }
})

const isMyTurnNext = computed(() => {
  // TODO get next turn ID from server
  return false;
})

</script>
