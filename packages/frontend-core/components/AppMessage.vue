<script setup lang="ts">
defineProps<{
  readonly modelValue: string;
}>();
</script>

<template>
  <div>New From App Message: {{ modelValue }}</div>
</template>
