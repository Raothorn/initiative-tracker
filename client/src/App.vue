<script setup lang="ts">
import { RouterView } from 'vue-router'
import PlayerHeader from './components/PlayerHeader.vue'
import TurnStateAlert from './components/TurnStateAlert.vue'
import AddEffectModal from './components/AddEffectModal.vue'
import MockStateControl from './components/MockStateControl.vue'
</script>

<template>
  <header>
    <div class="mt-3">
      <TurnStateAlert />
      <PlayerHeader />
    </div>
  </header>

  <RouterView />
  <MockStateControl />
  <AddEffectModal />
</template>

<style scoped></style>
