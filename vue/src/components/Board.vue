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
  console.log("hola");
  export default {
    components: {
    Cell
    },
    data() {
    return {
        board: Array(19).fill().map(() => Array(19).fill(null)),
        game: Game.new(),
        currentPlayer: 'X',
        finished: false,
    };
    },
    methods: {
    play(i, j) {
      if (this.finished) return;
      console.log(this.finished);
      console.log(this.game);
      if (this.board[i][j] !== null) return;
      if (this.currentPlayer == 'X'){
          if (this.game.place(i, j, 1) == true) {
              this.board[i][j] = this.currentPlayer;
              this.currentPlayer = this.currentPlayer === 'X' ? 'O' : 'X';
              console.log(this.currentPlayer);
          }
          else {
              console.log("Invalid move");
          }
      } else {
          if (this.game.place(i, j, 2) == true) {
              this.board[i][j] = this.currentPlayer;
              this.currentPlayer = this.currentPlayer === 'X' ? 'O' : 'X';
              console.log(this.currentPlayer);
          }
          else {
              console.log("Invalid move");
          }
      }
      if (this.game.check_win() == true) {
        this.finished = true;
        if (this.currentPlayer == 'X') {
          console.log("Player 2 wins");
        } else {
          console.log("Player 1 wins");
        }
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