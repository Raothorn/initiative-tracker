<template>
    <div class="fixed-bottom mb-5 text-center">
        <hr>
        <h6 class="mt-2">Control Panel</h6>
        <div class="row">
            <div class="col-3">
                
            </div>
            <div class="col">
                <div class="btn-group" role="group">
                    <button @click="rewindTurn" type="button" class="btn btn-outline-secondary">
                        <i class="fa-solid fa-backward"></i>
                    </button>
                    <button @click="" type="button" class="btn btn-outline-secondary">Turn</button>
                    <button @click="advanceTurn" type="button" class="btn btn-outline-secondary">
                        <i class="fa-solid fa-forward"></i>
                    </button>
                </div>
            </div>
            <div class="col-3">
                <button class="btn btn-outline-secondary">Player</button>
                <button class="btn btn-outline-secondary">DM</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import $socket from '@/socket';
import useGameState from '@/stores/gameState'

const gameState = useGameState()
function rewindTurn() {
}

function advanceTurn() {
    $socket.sendAction("advanceTurnAction", {}) 
}
</script>
