<template>
    <div class="modal" id="addPlayerEffectModal" tabindex="-1">
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h1 class="modal-title fs-5" id="exampleModalLabel">New Effect</h1>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <form>
                        <div class="mb-3">
                            <label class="form-label">Name</label>
                            <input type="text" class="form-control" placeholder="Effect name...">
                        </div>
                        <div class="mb-3">
                            <label class="form-label">Save</label>
                            <select class="form-select">
                                <option>Strength</option>
                                <option>Dexterity</option>
                                <option>Constitution</option>
                                <option>Intelligence</option>
                                <option>Wisdom</option>
                                <option>Charisma</option>
                            </select>
                        </div>
                        <div class="mb-3">
                            <label class="form-label">Duration</label>
                            <input type="number" class="form-control" placeholder="Number of rounds...">
                        </div>
                        <div class="mb-3">
                            <p>Color</p>
                            <label class="form-label">Presets</label>
                            <ul class="list-unstyled color-preset-picker">
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-brown">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-red">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-orange">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-yellow">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-lightgreen">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-darkgreen">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-lightblue">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-darkblue">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-purple">.</span></li>
                                <li><span class="badge rounded-pill effect-badge badge-swatch badge-pink">.</span></li>
                            </ul>
                            <label class="form-label">Custom</label>
                            <input type="color" class="form-control form-control-color" id="exampleColorInput" value="#563d7c" title="Choose your color">
                        </div>
                    </form>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-outline-secondary" data-bs-dismiss="modal">Cancel</button>
                    <button type="button" class="btn btn-outline-success" data-bs-dismiss="modal">Create</button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">

</script>

<style>
ul.color-preset-picker {
    display: grid;
    grid: auto / auto auto auto auto auto;
}

.effect-badge {
  color: transparent;
}

.badge-swatch {
  border: 4px solid;
  border-color: white;
  font-size: 1.2rem;
  margin-right: 0;
}

.badge-swatch:hover {
  border-color: #007BFF;
}

.badge-brown {
  background-color: sienna;
}
.badge-red {
  background-color: red;
}
.badge-orange {
  background-color: darkorange;
}
.badge-yellow {
  background-color: gold;
}
.badge-lightgreen {
  background-color: yellowgreen;
}
.badge-darkgreen {
  background-color: forestgreen;
}
.badge-lightblue {
  background-color: deepskyblue;
}
.badge-darkblue {
  background-color: royalblue;
}
.badge-purple {
  background-color: mediumpurple;
}
.badge-pink {
  background-color: hotpink;
}
</style>
  