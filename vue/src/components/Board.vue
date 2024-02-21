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
        board: Array(15).fill().map(() => Array(15).fill(null)),
        game: Game.new(),
        currentPlayer: 'X'
    };
    },
    methods: {
    play(i, j) {
        console.log(this.game);
        if (this.board[i][j] !== null) return;
        if (this.currentPlayer == 'X'){
            if (this.game.place(i, j, 0) == true) {
                this.board[i][j] = this.currentPlayer;
                this.currentPlayer = this.currentPlayer === 'X' ? 'O' : 'X';
                console.log(this.currentPlayer);
            }
            else {
                console.log("Invalid move");
            }
        } else {
            if (this.game.place(i, j, 1) == true) {
                this.board[i][j] = this.currentPlayer;
                this.currentPlayer = this.currentPlayer === 'X' ? 'O' : 'X';
                console.log(this.currentPlayer);
            }
            else {
                console.log("Invalid move");
            }
        }
        if (this.game.check_win() == true) {
            console.log("Player 1 wins");
        } else if (this.game.check_win() == true) {
            console.log("Player 2 wins");
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