<template>
  <main>
    <form>
      <div class="row mb-3">
        <label class="form-label">Player name</label>
        <input type="text" v-model="playerNameInput" class="form-control" placeholder="Player name..." />
      </div>
      <div class="row mb-3">
        <label class="form-label">Character name</label>
        <input type="text" class="form-control" placeholder="Character name..." />
      </div>
      <div class="row mb-3">
        <label class="form-label">Initiative bonus</label>
        <input type="number" class="form-control" placeholder="0" />
      </div>
    </form>
    <div class="row mb-3">
      <button @click="login" type="button" class="btn btn-outline-success">
        Join
      </button>
    </div>
  </main>
</template>

<script setup lang="ts">
import router from '@/router';
import useUser from '@/stores/currentUser';
import { ref, watch } from 'vue'

const userStore = useUser()
const playerNameInput = ref("Test name")

function login() {
  let input = playerNameInput.value
  if (input !== "") {
    userStore.login(input, 0)
  }
  watch(() => userStore.isLoggedIn, (loggedIn) => {
    if (loggedIn) {
      router.push('/init')
    }
  })
}

</script>
