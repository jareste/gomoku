<template>
    <button class="cell" :class="{ 'player-x': value === 'X', 'player-o': value === 'O' }" @click="$emit('click')">
      {{ "" }}
    </button>
  </template>
  
  <script>
  export default {
    props: ['value']
  };
  </script>
  
  <style scoped>
  .cell {
    width: 30px;
    height: 30px;
    margin: 5px;
  }
  .player-x {
    border-radius: 15%;
    background-color: #252222; /* Change to the color you want for player X */
  }
  .player-o {
    border-radius: 15%;
    background-color: #ac7676; /* Change to the color you want for player O */
  }
  </style>