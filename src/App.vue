<script setup lang="ts">
import { onMounted, ref } from "vue";
import RailSimulation from "./components/RailSimulation.vue";
import init, { greet } from "../wasm/pkg";

const wasmReady = ref(false);

onMounted(async () => {
    try {
        await init();
        wasmReady.value = true;
    } catch (error) {
        console.error(error);
        wasmReady.value = false;
    }
})

function button() {
    // WASMの準備ができていない場合は実行をガードする
    if (!wasmReady.value) {
        console.warn('WASM is not ready yet.')
        return
    }

    greet('hello')
}

</script>

<template>
    <main>
        <h1>鉄道台車・余弦引き込みシミュレーション</h1>
        <RailSimulation />
        <button :disabled="!wasmReady" @click="button"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">button</button>
    </main>
</template>

<style>
@import "tailwindcss";
</style>
