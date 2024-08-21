<template>
  <div class="tab-pane" id="tab-pane-init">
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
        <CombatantView :combatant=combatant></CombatantView>
      </tr>
    </tbody>
  </table>
  </div>
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

<style>
</style>
