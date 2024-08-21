<template>
  <td>{{ combatant.name }}</td>
  <td></td>
  <td>{{ combatant.initRoll + combatant.initMod }} ({{ combatant.initMod }})</td>
</template>

<script setup lang="ts">
import type { Combatant } from '../models/Combatant'

defineProps<{ combatant: Combatant }>()
</script>
