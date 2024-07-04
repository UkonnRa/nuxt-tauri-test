<script setup lang="ts">
import type { MenuItem } from "primevue/menuitem";

const expending = ref(false);
const width = computed(() => (expending.value ? 250 : 48));

const expendingIcon = computed(() => {
  return expending.value ? "fa-solid fa-chevron-left" : "fa-solid fa-chevron-right";
});

const toggleExpending = () => {
  expending.value = !expending.value;
};

const items: MenuItem[] = [
  {
    label: "Home",
    icon: "fa-solid fa-house",
  },
  {
    label: "Journal",
    icon: "fa-solid fa-book",
  },
  {
    label: "Accounts",
    icon: "fa-solid fa-building-columns",
  },
  {
    separator: true,
  },
  {
    label: "Q & A",
    icon: "fa-solid fa-circle-question",
  },
];
</script>

<template>
  <div class="fixed z-40">
    <Menu :model="items" :style="{ width: `${width}px` }" class="ease-in-out duration-150">
      <template #item="{ item, props }">
        <a v-ripple v-tooltip="item.label" v-bind="props.action">
          <span :class="item.icon" />
          <span v-if="expending" class="truncate">{{ item.label }}</span>
        </a>
      </template>

      <template #end>
        <Button
          v-tooltip="expending ? 'Shrink the menu' : 'Expand the menu'"
          text
          :icon="expendingIcon"
          class="w-full"
          @click="toggleExpending"
        />
      </template>
    </Menu>
  </div>
</template>

<style lang="scss">
.p-menu {
  min-width: fit-content;
  transition-property: width;
}
</style>
