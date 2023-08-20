<script>
import { invoke } from '@tauri-apps/api';

export default {
  data() {
    return {
      hello: ''
    };
  },
  created() {
    invoke('greet', { name: 'World' })
      .then((response) => {
        this.hello = response;
      });
  }
};
</script>

<template>
  <header>
    
    <div class="wrapper">
      <div>Do you want to do a prescan ?</div>
      <div>{{ hello }}</div>
      <button> Charger </button>
    </div>
  </header>

</template>

<style scoped>

</style>
