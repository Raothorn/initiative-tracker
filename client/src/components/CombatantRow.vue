<template>
  <tr :class="{ 'table-secondary': isCurrentTurn }">
    <td>{{ combatant.name }}</td>
    <td></td>
    <td>{{ combatant.initRoll + combatant.initMod }} ({{ combatant.initMod }})</td>
  </tr>
</template>

<script setup lang="ts">
import useGameState from '@/stores/gameState'
import type { Combatant } from '../models/Combatant'
import { computed } from 'vue'

const props = defineProps<{ combatant: Combatant }>()

const gameState = useGameState()

const isCurrentTurn = computed(() => {
  let encounter = gameState.encounter;
  if (encounter) {
    return encounter.currentTurnId == props.combatant.id
  } 
  else {
    return false
  }
})
</script>
