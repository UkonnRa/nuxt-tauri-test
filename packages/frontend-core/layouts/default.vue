<script setup lang="ts">
import { useElementBounding } from "@vueuse/core";

const elToolbar = ref();
const elSideMenu = ref();

const { height: toolbarHeight } = useElementBounding(elToolbar);
const { width: elSideMenuWidth } = useElementBounding(elSideMenu);

const navigateToHome = () => {
  return navigateTo({
    name: "index",
  });
};
</script>

<template>
  <div>
    <ClientOnly>
      <Toolbar ref="elToolbar" class="fixed top-0 left-0 right-0 z-50 rounded-none">
        <template #start>
          <div class="flex flex-col select-none cursor-pointer" @click="navigateToHome">
            <strong class="text-2xl">White Rabbit</strong>
            <small class="italic font-light">Alice's Wonderland</small>
          </div>
        </template>

        <template #center>
          <slot name="center" />
        </template>

        <template #end>
          <div class="flex gap-2 items-center">
            <IconField>
              <InputIcon>
                <i class="fa-solid fa-magnifying-glass" />
              </InputIcon>
              <InputText placeholder="Search" />
            </IconField>
            <Button label="Save" />
          </div>
        </template>
      </Toolbar>
    </ClientOnly>

    <main :style="{ paddingTop: `${toolbarHeight}px` }">
      <AppSideMenu ref="elSideMenu" :style="{ paddingTop: `${toolbarHeight}px` }" />
      <div :style="{ paddingLeft: `${elSideMenuWidth}px` }">
        <slot />
      </div>
    </main>
  </div>
</template>
