<script setup lang="ts">
const route = useRoute();
</script>

<template>
  <NuxtLayout name="default">
    <template #center>
      <span>Journal: {{ route.params.id }}</span>
    </template>

    <slot />
  </NuxtLayout>
</template>
