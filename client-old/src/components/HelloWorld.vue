<template>
  <div class="tab-content">
    <InitiativeTable></InitiativeTable>
    <EffectsList></EffectsList>
    <InfoView></InfoView>
  </div>
  <AddPlayerEffectModal></AddPlayerEffectModal>
</template>

<script setup lang="ts">
import InitiativeTable from "./InitiativeTable.vue"
import EffectsList from "./EffectsList.vue"
import InfoView from "./InfoView.vue"
import AddPlayerEffectModal from "./AddPlayerEffectModal.vue";
</script>
