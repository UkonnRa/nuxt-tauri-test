<script setup lang="ts">
const props = defineProps<{
  readonly modelValue: number;
}>();

const config = useRuntimeConfig();
const id = ref(1);
const { data, status } = useFetch("/api/hello", {
  key: `${props.modelValue}`,
  query: {
    id: id,
  },
});
</script>

<template>
  <div class="flex flex-col gap-2 w-full">
    <div><Button :label="`ID: ${id}`" @click="id++" /></div>
    <ProgressBar v-if="status === 'pending'" mode="indeterminate" style="height: 4px" />
    <div v-else class="flex flex-col gap-2">
      <div>
        <strong>Calling API:</strong>
        <Chip>{{ config.public.apiBase }}</Chip>
      </div>

      <div>
        <strong>Data Loaded:</strong>
        <code>
          <pre>{{ JSON.stringify(data, null, 2) }}</pre>
        </code>
      </div>
    </div>
  </div>
</template>
