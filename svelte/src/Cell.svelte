<script>
  export let cell;
</script>

<button class="cell" disabled={cell !== 0}>
    {cell}
</button>

<style>
  .cell {
    width: 30px;
    height: 30px;
    margin: 10px;
  }
</style>