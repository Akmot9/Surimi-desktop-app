<script>
import { invoke } from '@tauri-apps/api';

export default {
  data() {
    return {
      hello: '',
      uwu: ''
    };
  },
  created() {
    invoke('greet', { name: 'World' })
      .then((response) => {
        this.hello = response;
        console.log(response)
      });
      console.log("created: ")
  },
  methods: {
    prescan() {
      console.log("chargement...")
      invoke('okok', { name: 'World' })
      .then((response) => {
        console.log(response)
        this.uwu = response;
      });
      console.log("methodes: ")
      
    }
  }
};
</script>

<template>
  <header>
    
    <div class="wrapper">
      <div>Do you want to do a prescan ?</div>
      <div>{{ hello }}</div>
      <button @click="prescan"> Charger </button>
    </div>
  </header>

</template>

<style scoped>

</style>
