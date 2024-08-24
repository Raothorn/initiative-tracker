<template>
  <main>
    <div v-if="gameState.encounter">
      <table class="table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Effects</th>
            <th>Init</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="combatant in gameState.encounter.combatants">
            <CombatantRow :combatant="combatant"></CombatantRow>
          </template>
        </tbody>
      </table>
    </div>
    <div v-else>
      Encounter phase hasn't started
    </div>
  </main>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import CombatantRow from '../components/CombatantRow.vue'
import useGameState from '@/stores/gameState'

const gameState = useGameState()

</script>
