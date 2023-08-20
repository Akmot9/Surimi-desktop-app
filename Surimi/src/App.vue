<script setup>
import { invoke } from '@tauri-apps/api'

invoke('greet', { name: 'World' })
  // `invoke` renvoie une promesse (Promise)
  .then((response) => console.log(response))
</script>

<template>
  <header>
    
    <div class="wrapper">
      <div>hello</div>
      <div>{hello}</div>
    </div>
  </header>

</template>

<style scoped>

</style>
