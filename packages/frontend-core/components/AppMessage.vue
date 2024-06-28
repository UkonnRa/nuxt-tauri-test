<script setup lang="ts">
defineProps<{
  readonly modelValue: string;
}>();
</script>

<template>
  <div class="flex flex-col gap-1">
    <span>New From App Message:</span>
    <Message>{{ modelValue }}</Message>
  </div>
</template>
