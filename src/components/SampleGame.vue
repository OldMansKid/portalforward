<script setup lang="ts">
import { onMounted } from 'vue'
import Phaser from 'phaser'
import { SampleGame } from '@/stores/SampleGame'

onMounted(() => {
  const config: Phaser.Types.Core.GameConfig = {
    type: Phaser.AUTO,
    width: 800,
    height: 600,
    scene: [SampleGame],
    parent: 'game-container',
    physics: {
      default: 'arcade',
      arcade: {
        gravity: { x: 0, y: 200 },
        debug: false,
      },
    },
  }

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
