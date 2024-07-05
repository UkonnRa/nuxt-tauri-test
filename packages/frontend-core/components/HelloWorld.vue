<script setup lang="ts">
defineProps<{
  readonly modelValue: number;
}>();

const config = useRuntimeConfig();
const id = ref(1);
const command = computed(() => ({
  create: {
    name: "New Journal: " + id.value,
    description: "New Desc",
    unit: "CNY",
    tags: ["tag1", "tag2"],
  },
}));
watch(command, (newCommand) => {
  console.log("New Command:", newCommand);
});

const { $journalClient } = useNuxtApp();

const { data: ids, status: idsStatus } = $journalClient.handleCommand(command);

const query = computed(() => ({
  id: ids.value,
}));
const { data: journals, status: journalsStatus } = $journalClient.findAll(query);
</script>

<template>
  <Card>
    <template #title>Data</template>
    <template #content>
      <ProgressBar
        v-if="idsStatus === 'pending' || journalsStatus === 'pending'"
        mode="indeterminate"
        class="h-2"
      />
      <div v-else class="flex flex-col gap-2">
        <div>
          <strong>Calling API:</strong>
          <Chip>{{ config.public.apiBase }}</Chip>
        </div>

        <div>
          <strong>Data Loaded:</strong>
          <code>
            <pre>{{ JSON.stringify(ids, null, 2) }}</pre>
            <pre>{{ JSON.stringify(journals, null, 2) }}</pre>
          </code>
        </div>
      </div>
    </template>
    <template #footer>
      <Button :label="`ID: ${id}`" @click="id++" />
    </template>
  </Card>
</template>
