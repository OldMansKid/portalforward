<script setup lang="ts">
import { onMounted } from 'vue'
import Phaser from 'phaser'
import { createGameConfig } from '@/stores/SampleGameTools'
import { SampleGameTitle } from '@/stores/SampleGameTitle'

onMounted(() => {
  const config: Phaser.Types.Core.GameConfig = createGameConfig([SampleGameTitle], 'game-container')

  new Phaser.Game(config)
})
</script>

<template>
  <div id="game-container" style="width: 800px; height: 600px"></div>
</template>

<style scoped>
#game-container {
  display: flex;
  justify-content: center;
  align-items: center;
  max-width: 800px;
}
</style>
