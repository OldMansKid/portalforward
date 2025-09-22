<script lang="ts" setup>
import SampleGame from '@/components/SampleGame.vue'
</script>

<template>
  <div class="game-view">
    <label>Game View</label>
    <SampleGame />
  </div>
</template>

<style scoped>
.game-view {
  padding: 2rem;
  display: flex;
  align-items: center;
  flex-direction: column;
}
</style>
