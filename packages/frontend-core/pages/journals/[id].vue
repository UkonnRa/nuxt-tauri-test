<script setup lang="ts">
definePageMeta({
  layout: "journal",
});

const route = useRoute();
</script>

<template>
  <div>Journal: {{ route.params.id }}</div>
</template>
