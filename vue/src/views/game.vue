<script setup lang="ts">
import Board from '../components/Board.vue'
</script>


<template>
    <main>
      <Board />
    </main>
  </template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
