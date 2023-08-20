<script>
import { invoke } from '@tauri-apps/api';

export default {
  data() {
    return {
      hello: '',
      nbs_of_files: ''
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
      invoke('list_files_execution', { name: 'World' })
      .then((response) => {
        console.log(response)
        this.nbs_of_files = response;
      });
      console.log("methodes: ")
      
    }
  }
};
</script>

<template>
  <div class="app">
    <header class="header">
      <h1 class="title">Surimi Integrity Checker</h1>
      <p class="subtitle">A simple Rust CLI app to make integrity reports of your computer.</p>
    </header>

    <main class="main">
      <div class="options">
        <button @click="prescan" class="button">Perform Prescan</button>
        <p class="hint">Perform a quick scan of your computer's files to generate integrity reports.</p>
      </div>

      <div class="result" v-if="nbs_of_files">
        <h2 class="result-title">Prescan Result</h2>
        <pre class="result-text">{{ nbs_of_files }}</pre>
      </div>
    </main>
  </div>
</template>


<style scoped>
.app {
  font-family: Arial, sans-serif;
  text-align: center;
  margin: 0 auto;
  padding: 20px;
  max-width: 800px;
}

.header {
  background-color: #ef705a;
  color: white;
  padding: 20px;
  border-radius: 10px 10px 0 0;
}

.title {
  margin: 0;
  font-size: 24px;
}

.subtitle {
  margin: 10px 0;
  font-size: 16px;
}

.main {
  padding: 20px;
  border-radius: 0 0 10px 10px;
  background-color: #f9f9f9;
}

.options {
  margin-bottom: 20px;
}

.button {
  background-color: #ef705a;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
}

.hint {
  font-size: 14px;
  color: #777;
}

.result {
  background-color: white;
  padding: 20px;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.result-title {
  font-size: 20px;
  margin-top: 0;
}

.result-text {
  white-space: pre-wrap;
}
</style>

