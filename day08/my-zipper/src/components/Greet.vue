<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const age = ref(99);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   let age = 100
  greetMsg.value = await invoke("greet", { name: name.value, age: +age.value });
  console.log(age.value)
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <input id="greet-value" v-model="age" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
