<template>
    <div class="board">
      <div class="move-count">Move count: {{ moveCount }}</div>
      <div class="current-player">Current player: {{ currentPlayer }}</div>
      <div class="captured1">Captured1: {{ captured1 }}</div>
      <div class="captured2">Captured2: {{ captured2 }}</div>
      <div v-for="(row, i) in board" :key="i" class="row">
          <div v-for="(cell, j) in row" :key="j" @click="play(i, j)" 
               :class="['cell', cell === 'X' ? 'red' : cell === 'O' ? 'black' : '']">
          </div>
        </div>
    </div>
</template>
  
  <script>
  import Cell from './Cell.vue';
  import { Game } from 'rust';
  console.log("holaIA");
  function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

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
            captured2: 0,
            initialized: false
        };
    },
    methods: {
      // updateBoard() {
      //   let flatMap = this.game.get_map();
      //   for (let i = 0; i < 19; i++) {
      //     for (let j = 0; j < 19; j++) {
      //       let value = flatMap[i * 19 + j];
      //       if (value == 1) {
      //         this.board[i][j] = 'X';
      //       } else if (value == 2) {
      //         this.board[i][j] = 'O';
      //       } else {
      //         this.board[i][j] = null;
      //       }
      //     }
      //   }
      // },
      play(i, j) {
        console.log("play on position:", i, j);
        if (this.initialized == false) {
          console.log("initializing..........................................................................................");
          this.game.start_IA();
          this.board[9][9] = 'X';
          this.initialized = true;
        }
        if (this.finished) return;
        if (this.board[i][j] !== null) return;

        if (this.game.place(i, j, 2) == true) {
          this.moveCount++;
          console.log(this.currentPlayer);
        } else {
          console.log("Invalid move");
        }

        if (this.game.check_win() == true) {
          this.finished = true;
          console.log("Player wins");
        } else {
          let t0 = performance.now();
          let iaMove = this.game.place_ia();
          let t1 = performance.now();
          console.log("Call to place_ia took " + ((t1 - t0) / 1000) + " seconds.");
          // this.board[iaMove.get_x()][iaMove.get_y()] = 'X';

          if (this.game.check_win() == true) {
            this.finished = true;
            console.log("IA wins");
          }
          // this.playIA();
        }
        this.captured1 = this.game.get_captured1();
        this.captured2 = this.game.get_captured2();
        // this.updateBoard();

        let flatMap = this.game.get_map();
        for (let i = 0; i < 19; i++) {
          for (let j = 0; j < 19; j++) {
            let value = flatMap[i * 19 + j];
            if (value == 1) {
              this.board[i][j] = 'X';
            } else if (value == 2) {
              this.board[i][j] = 'O';
            } else {
              this.board[i][j] = null;
            }
          }
        }
      },
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
  .cell {
    width: 30px;
    height: 30px;
    border: 1px solid #000;
    display: inline-block;
    line-height: 30px;
    text-align: center;
  }
  .red {
    background-color: red;
  }
  .black {
    background-color: black;
  }
  </style>