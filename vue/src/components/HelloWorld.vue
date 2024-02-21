<script setup lang="ts">
defineProps<{
  msg: string
}>()
import init from 'rust';
init().then(() => {
  console.log('init wasm-pack');
});
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <h3>
      Built with Vue 3 and Rust. Made with love by 
      <a href="https://github.com/bielaltes" target="_blank" rel="noopener">@baltes-g</a> and
      <a href="https://github.com/jareste" target="_blank" rel="noopener">@jareste-</a>.
    </h3>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
