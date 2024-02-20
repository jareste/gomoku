<script>
import Board from './Board.svelte';
export let bindings;
</script>

<main>
  <div class="center">
    <h1>Gomoku</h1>
  </div>
  <div class="center">
    <Board />
  </div>
</main>

<style>
  .center {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>