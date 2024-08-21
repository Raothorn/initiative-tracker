<script setup lang="ts">
import { RouterView } from 'vue-router'
import PlayerHeader from './components/PlayerHeader.vue'
</script>

<template>
  <header>
    <div class="wrapper">
      <PlayerHeader />
    </div>
  </header>

  <RouterView />
</template>

<style scoped></style>
