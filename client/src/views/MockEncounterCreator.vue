<template>
  <main>
    <h3 class="section-header">Players</h3>
    <template v-for="player in gameState.players">
      <div @click="togglePlayerSelected(player.guid)" class="card w-100 d-flex justify-content-between flex-row"
        :class="{ 'selected-player': isPlayerSelected(player.guid) }">
        <div class="p-2">{{ player.name }}</div>
        <div class="p-2">{{ player.initiativeBonus }}</div>
      </div>
    </template>
    <button @click="createEncounter">Create Encounter</button>
  </main>
</template>

<script setup lang="ts">
import $socket from '@/socket';
import useGameState from '@/stores/gameState'
import { computed, ref, type Ref } from 'vue'

const gameState = useGameState()
const selectedPlayerGuids: Ref<string[]> = ref([])

function togglePlayerSelected(guid: string) {
  let index = selectedPlayerGuids.value.indexOf(guid)
  if (index == -1) {
    selectedPlayerGuids.value.push(guid)
  } else {
    selectedPlayerGuids.value.splice(index, 1)
  }
}

function isPlayerSelected(guid: string) {
  return selectedPlayerGuids.value.indexOf(guid) != -1
}

function createEncounter() {
  let actionData = { selectedPlayerGuids: selectedPlayerGuids.value }
  $socket.sendAction("startEncounterAction", actionData)
}
</script>

<style scoped>
.card {
  margin-bottom: 10px;
}

.card:hover {
  border-width: 2px;
}

.selected-player {
  background-color: green;
}
</style>
