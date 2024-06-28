<script setup lang="ts">
definePageMeta({
  layout: "journal",
});

const ids = [1, 2, 3, 4];
</script>

<template>
  <div class="flex flex-col gap-2">
    <strong>All Journals:</strong>
    <div class="flex flex-col gap-1">
      <NuxtLink v-for="id in ids" :key="id" :to="{ name: 'journals-id', params: { id: id } }"
        >Journal {{ id }}</NuxtLink
      >
    </div>
  </div>
</template>
