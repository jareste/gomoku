<template>
    <div class="board">
      <div v-for="(row, i) in board" :key="i" class="row">
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
            ifs: 1,
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
      if (this.board[i][j] !== null) return;
      console.log(9+this.ifs, this.ifs);
      this.game.place(9, 9+this.ifs, 1);
      this.board[9][9+this.ifs] = 'X';
      if (this.game.place(i, j, 2) == true) {
        this.board[i][j] = 'O';
        console.log(this.currentPlayer);
      } else {
        console.log("Invalid move");
      }
      if (this.game.check_win() == true) {
        this.finished = true;
        if (this.currentPlayer == 'X') {
          console.log("Player wins");
        } else {
          console.log("IA wins");
        }
      }
      this.ifs = this.ifs + 1;
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