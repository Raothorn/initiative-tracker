<script setup lang="ts">
import { RouterView } from 'vue-router'
import PlayerHeader from './components/PlayerHeader.vue'
import TurnStateAlert from './components/TurnStateAlert.vue'
import AddEffectModal from './components/AddEffectModal.vue'
import MockStateControl from './components/MockStateControl.vue'
import {$socket} from './socket'
import useGameState from './stores/gameState'
import { onBeforeUnmount, onMounted, onUnmounted } from 'vue'

const gameState = useGameState()

onMounted(()=> {
  $socket.connect()
})

onBeforeUnmount(()=> {
  $socket.disconnect();
})
</script>

<template>
  <div v-if="gameState.hasInitialized">
    <header>
      <div class="mt-3">
        <TurnStateAlert />
        <PlayerHeader />
      </div>
    </header>

    <RouterView />
    <MockStateControl />
    <AddEffectModal />
  </div>
  <div v-else>
    Waiting on connection to server..
  </div>

</template>

<style scoped></style>
