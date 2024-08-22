<script setup lang="ts"></script>

<template>
  <main>
    <form>
      <div class="mb-3">
        <label class="form-label">Character Name</label>
        <input
          type="text"
          class="form-control"
          placeholder="Change your character name..."
          value="Billie"
        />
      </div>
      <div class="mb-3">
        <label class="form-label">Player Name</label>
        <input
          type="text"
          class="form-control"
          placeholder="Change your player name..."
          value="Tanya"
        />
      </div>
      <div class="mb-3">
        <label class="form-label">Initiative Bonus</label>
        <input
          type="number"
          class="form-control"
          placeholder="Change your initiative bonus..."
          value="1"
        />
      </div>
      <span class="btn btn-outline-secondary">Save</span>
    </form>
  </main>
</template>
