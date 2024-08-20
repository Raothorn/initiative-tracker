<template>
  <p> Name:  {{ combatant.name }} HP: {{ combatant.currentHitpoints }} </p>
</template>

<script setup lang="ts">
import { Combatant } from 'share'

defineProps<{ combatant: Combatant }>()

</script>

<style>
</style>
