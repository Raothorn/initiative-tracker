<script setup lang="ts">
import { RouterLink } from 'vue-router'
</script>

<template>
  <h1 class="mt-3">Player 1 <span class="float-right">Init</span></h1>
  <nav class="nav nav-tabs mb-3">
    <RouterLink to="/init" class="nav-link link-dark"><i class="fa-solid fa-house"></i></RouterLink>
    <RouterLink to="/effects" class="nav-link link-dark"
      ><i class="fa-solid fa-repeat"></i
    ></RouterLink>
    <RouterLink to="/info" class="nav-link link-dark"><i class="fa-solid fa-user"></i></RouterLink>
  </nav>
</template>
