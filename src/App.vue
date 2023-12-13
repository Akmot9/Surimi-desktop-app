<script>
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

export default {
    data() {
        return {
            count: 0,
        };
    },
    mounted() {
        listen('file_count', (event) => {
            this.count = event.payload;
        });
    },
    methods: {
        async StartCount() {
            try {
                console.log('getCount');
                await invoke('get_number_of_files');
            } catch (error) {
                console.error(error);
            }
        },
    },
};
</script>

<template>
    <button @click="StartCount"></button>
    <div>
        Nombre de fichiers: {{ count }}
    </div>
</template>

<style scoped>

</style>
