import type { AsyncData } from "#app";

export interface HelloClient {
  hello(name: Ref<string>): AsyncData<unknown, unknown>;
}

declare module "#app" {
  interface NuxtApp {
    $helloClient: HelloClient;
  }
}

declare module "vue" {
  interface ComponentCustomProperties {
    $helloClient: HelloClient;
  }
}

export {};
