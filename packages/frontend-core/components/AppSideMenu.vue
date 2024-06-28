<script setup lang="ts">
const expending = ref(false);
const width = computed(() => (expending.value ? 100 : 250));

const toggleExpending = () => {
  expending.value = !expending.value;
};
</script>

<template>
  <div
    :style="{ width: `${width}px` }"
    class="fixed top-0 left-0 z-40 h-screen ease-in-out duration-150"
  >
    <div>Hello 1</div>
    <div>Hello 2</div>
    <div>Hello 3</div>
    <div>Hello 4</div>
    <div>Hello 5</div>
    <div>Hello 6</div>
    <div>Hello 7</div>
    <div>Hello 8</div>
    <div>Hello 9</div>
    <div>Hello 10</div>
    <Button icon="fa-solid fa-warning" class="w-full" @click="toggleExpending" />
  </div>
</template>

<style lang="scss">
div {
  transition-property: width;
}
</style>
