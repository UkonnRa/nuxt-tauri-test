import { type WriteService, type ReadAggregate, WriteAggregate } from ".";
import type { AsyncData } from "#app";

export class Journal extends WriteAggregate {
  readonly name: string;
  readonly description: string;
  readonly unit: string;
  readonly tags: string[];

  constructor(
    id: string,
    createdDate: string,
    version: number,
    name: string,
    description: string,
    unit: string,
    tags: string[],
  ) {
    super(id, createdDate, version);
    this.name = name;
    this.description = description;
    this.unit = unit;
    this.tags = tags;
  }
}

export interface JournalQuery {}

export type JournalCommand = object;

export interface JournalService extends WriteService<Journal, JournalQuery, JournalCommand> {}

export interface JournalClient {
  findById(
    id: MaybeRef<string>,
  ): AsyncData<[Journal, Map<string, ReadAggregate>] | undefined, unknown>;
  findAll(
    query: MaybeRef<JournalQuery>,
  ): AsyncData<[Journal[], Map<string, ReadAggregate>], unknown>;
  handleCommand(command: MaybeRef<JournalCommand>): AsyncData<string[], unknown>;
}
