<template>
    <div class="board">
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