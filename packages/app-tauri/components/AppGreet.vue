<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <form class="flex gap-2" @submit.prevent="greet">
    <FloatLabel>
      <InputText id="name" v-model="name" />
      <label for="name">Enter a name...</label>
    </FloatLabel>
    <Button label="Submit" type="submit" icon="pi pi-check" />
  </form>

  <AppMessage :model-value="greetMsg" />
</template>
