<template>
  <main>
    <table class="table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Effects</th>
          <th>HP</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="combatant in combatants">
          <CombatantRow :combatant="combatant"></CombatantRow>
        </tr>
      </tbody>
    </table>
  </main>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import CombatantRow from '../components/CombatantRow.vue'
import useGameState from '@/stores/app'

const gamestateStore = useGameState()

// Retrieves combatants in order of initiative
const combatants = computed(() => {
  let combatants = gamestateStore.gameState.combatants
  combatants.sort((a, b) => a.initRoll + a.initMod - (b.initRoll + b.initMod))
  return combatants
})
</script>
