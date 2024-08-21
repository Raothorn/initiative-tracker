<template>
  <td>{{ combatant.name }}</td>
  <td></td>
  <td>{{ combatant.currentHitpoints }}</td>
</template>

<script setup lang="ts">
import { Combatant } from 'share'

defineProps<{ combatant: Combatant }>()

</script>

<style>
</style>
