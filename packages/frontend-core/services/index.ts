export abstract class ReadAggregate {
  readonly id: string;

  protected constructor(id: string) {
    this.id = id;
  }
}

export abstract class WriteAggregate extends ReadAggregate {
  readonly createdDate: string;
  readonly version: number;

  protected constructor(id: string, createdDate: string, version: number) {
    super(id);
    this.createdDate = createdDate;
    this.version = version;
  }
}

export interface ReadService<A, Q> {
  findAll(query: Q): Promise<[A[], Map<string, ReadAggregate>]>;
  findById(id: string): Promise<[A, Map<string, ReadAggregate>] | undefined>;
}

export interface WriteService<A, Q, C> extends ReadService<A, Q> {
  handleCommand(command: C): Promise<string[]>;
}
