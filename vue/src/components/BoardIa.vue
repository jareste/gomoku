<template>
    <div class="board">
      <div class="move-count">Move count: {{ moveCount }}</div>
      <div class="current-player">Current player: {{ currentPlayer }}</div>
      <div class="captured1">Captured1: {{ captured1 }}</div>
      <div class="captured2">Captured2: {{ captured2 }}</div>
      <div v-for="(row, i) in board" :key="i" class="row">
        <!-- <span class="row-number">{{ i + 1 }}</span> -->
        <Cell v-for="(cell, j) in row" :key="j" @click="play(i, j)" :value="cell" />
      </div>
    </div>
</template>
  
  <script>
  import Cell from './Cell.vue';
  import { Game } from 'rust';
  console.log("holaIA");
  export default {
    components: {
    Cell
    },
    data() {
        return {
            board: Array(19).fill().map(() => Array(19).fill(null)),
            game: Game.new(),
            currentPlayer: 'O',
            finished: false,
            moveCount: 0,
            captured1: 0,
            captured2: 0
        };
    },
    created() {
      this.board[9][9] = 'X';
    },
    methods: {
      play(i, j) {
        if (this.finished) return;
        console.log(this.finished);
        console.log(this.game);
        console.log("placed at:", i, j);
        if (this.board[i][j] !== null) return;

        if (this.game.place(i, j, 2) == true) {
          this.board[i][j] = 'O';
          this.moveCount++;
          console.log(this.currentPlayer);
        } else {
          console.log("Invalid move");
        }

        if (this.game.check_win() == true) {
          this.finished = true;
          console.log("Player wins");
        } else {
          this.playIA();
        }
        this.captured1 = this.game.get_captured1();
        this.captured2 = this.game.get_captured2();
      },
      playIA() {
        let t0 = performance.now();
        let iaMove = this.game.place_ia();
        let t1 = performance.now();
        console.log("Call to place_ia took " + ((t1 - t0) / 1000) + " seconds.");
        this.board[iaMove.get_x()][iaMove.get_y()] = 'X';

        if (this.game.check_win() == true) {
          this.finished = true;
          console.log("IA wins");
        }
      }
    }
};
  </script>
  
  <style scoped>
  .board {
    display: flex;
    flex-direction: column;
  }
  .row {
    display: flex;
  }
  </style>