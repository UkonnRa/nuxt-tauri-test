<script setup lang="ts">
defineProps<{
  readonly modelValue: number;
}>();

const config = useRuntimeConfig();
const id = ref(1);
const { $helloClient } = useNuxtApp();
const idStr = computed(() => `${id.value}`);

const { data, status } = $helloClient.hello(idStr);
</script>

<template>
  <Card>
    <template #title>Data</template>
    <template #content>
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
    </template>
    <template #footer>
      <Button :label="`ID: ${id}`" @click="id++" />
    </template>
  </Card>
</template>
