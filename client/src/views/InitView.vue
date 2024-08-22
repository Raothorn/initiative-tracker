<template>
  <main>
    <table class="table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Effects</th>
          <th>Init</th>
        </tr>
      </thead>
      <tbody>
        <template v-for="combatant in gameStateStore.sortedCombatants">
          <CombatantRow :combatant="combatant"></CombatantRow>
        </template>
      </tbody>
    </table>
  </main>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import CombatantRow from '../components/CombatantRow.vue'
import useGameState from '@/stores/gameState'

const gameStateStore = useGameState()
</script>
