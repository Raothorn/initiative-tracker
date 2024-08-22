<template>
  <main>
    <div class="d-grid">
      <button
        type="button"
        class="btn btn-outline-success"
        data-bs-toggle="modal"
        data-bs-target="#addEffectModal"
      >
        Add Effect <i class="fa-solid fa-plus"></i>
      </button>
    </div>
    <hr />
    <h4>Your Effects</h4>
    <table class="table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Color</th>
          <th>Save</th>
          <th>Duration</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Confusion</td>
          <td></td>
          <td><span class="badge text-bg-secondary">Int</span></td>
          <td>3</td>
        </tr>
        <tr>
          <td>Bless</td>
          <td></td>
          <td></td>
          <td>3</td>
        </tr>
        <tr>
          <td>Bane</td>
          <td></td>
          <td><span class="badge text-bg-secondary">Wis</span></td>
          <td>3</td>
        </tr>
      </tbody>
    </table>
  </main>
</template>

<script setup lang="ts"></script>
