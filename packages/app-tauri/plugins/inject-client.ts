import { invoke } from "@tauri-apps/api/core";
import type { AsyncData } from "nuxt/app";

export default defineNuxtPlugin(() => {
  return {
    provide: {
      helloClient: {
        hello(name: Ref<string>): AsyncData<unknown, unknown> {
          return useAsyncData(
            `hello:${Math.random()}`,
            () => invoke<unknown>("greet", { name: name.value }),
            { watch: [name] },
          );
        },
      },
    },
  };
});
