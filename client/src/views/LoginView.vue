<template>
  <main>
    <input type="text" label="Username " v-model="usernameInput"></input>
    <button @click="login">Login</button>
  </main>
</template>

<script setup lang="ts">
import useUser from '@/stores/currentUser';
import { ref } from 'vue'

const userStore = useUser()
const usernameInput = ref("")

function login() {
  let input = usernameInput.value
  if (input !== "") {
    userStore.login(input)
  }
}
</script>