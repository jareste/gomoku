<script  context="module">
  import Cell from './Cell.svelte';
  export let currentPlayer = '1';
  let board = Array(19).fill().map(() => Array(19).fill(0));
  
  function play(event, row, col) {
    if (board[row][col] !== null) return;
    board[row][col] = currentPlayer;
    currentPlayer = currentPlayer === '1' ? '2' : '1';
    console.log("slidvhdksfhv");
    const button = event.target;
    button.textContent = currentPlayer;
  }
</script>

<div class="board">
  {#each board as row, i}
    <div class="row">
      {#each row as cell, j}
        <Cell {cell} on:click={() => play(i, j)}/>
      {/each}
    </div>
  {/each}
</div>

<style>
  .board {
    display: flex;
    flex-direction: column;
  }
  .row {
    display: flex;
  }
</style>