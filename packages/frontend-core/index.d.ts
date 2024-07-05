import type { JournalClient } from "./services/journal";

declare module "#app" {
  interface NuxtApp {
    $journalClient: JournalClient;
  }
}

declare module "vue" {
  interface ComponentCustomProperties {
    $journalClient: JournalClient;
  }
}

export {};
