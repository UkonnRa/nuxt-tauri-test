import type { ReadAggregate } from "@white-rabbit/frontend-core/services";
import type {
  Journal,
  JournalClient,
  JournalCommand,
  JournalQuery,
} from "@white-rabbit/frontend-core/services/journal";
import type { AsyncData } from "nuxt/app";
import { v7 as uuidv7 } from "uuid";

export default defineNuxtPlugin(() => {
  return {
    provide: {
      journalClient: {
        findById(
          id: MaybeRef<string>,
        ): AsyncData<[Journal, Map<string, ReadAggregate>] | undefined, unknown> {
          const unwrapped = toValue(id);
          return useAsyncData(
            `journals:${unwrapped}`,
            () => $fetch("/api/journal", { query: { id: unwrapped } }),
            { watch: isRef(id) ? [id] : undefined },
          );
        },
        findAll(
          query: MaybeRef<JournalQuery>,
        ): AsyncData<[Journal[], Map<string, ReadAggregate>], unknown> {
          const unwrapped = toValue(query);
          return useAsyncData(
            `journals:${JSON.stringify(unwrapped)}`,
            () => $fetch("/api/journals", { query: { query: unwrapped } }),
            { watch: isRef(query) ? [query] : undefined },
          );
        },
        handleCommand(command: MaybeRef<JournalCommand>): AsyncData<string[], unknown> {
          // For update methods, the key should always be changed, to ignore the cache
          return useAsyncData(
            uuidv7(),
            () => $fetch("/api/journals", { method: "POST", body: toValue(command) }),
            { watch: isRef(command) ? [command] : undefined },
          );
        },
      } satisfies JournalClient,
    },
  };
});
