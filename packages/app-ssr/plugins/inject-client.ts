import type { AsyncData } from "nuxt/app";

export default defineNuxtPlugin(() => {
  return {
    provide: {
      helloClient: {
        hello(name: Ref<string>): AsyncData<unknown, unknown> {
          return useAsyncData(
            `hello:${Math.random()}`,
            () => $fetch("/api/hello", { query: { name: `${name.value} from SSR` } }),
            { watch: [name] },
          );
        },
      },
    },
  };
});
