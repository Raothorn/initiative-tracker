<template>
  <span v-for="combatant in combatants">
    <CombatantView :combatant=combatant></CombatantView>
  </span>
</template>

<script setup lang="ts">
import CombatantView from "./CombatantView.vue"
import useGameState from "@/stores/app";

const gamestateStore = useGameState()

// Retrieves combatants in order of initiative
const combatants = computed(() => {
  let combatants = gamestateStore.gameState.combatants
  combatants.sort((a, b) => a.initiative - b.initiative)
  return combatants
})

</script>
