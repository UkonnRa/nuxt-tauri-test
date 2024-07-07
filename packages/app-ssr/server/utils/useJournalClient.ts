import {
  JournalServiceClient,
  FindByIdRequest,
  type Journal as JournalProto,
  FindAllRequest,
  HandleCommandRequest,
} from "../proto/gen/whiterabbit/journal/v1/journal";
import { ChannelCredentials } from "@grpc/grpc-js";
import type { ReadAggregate } from "@white-rabbit/frontend-core/services";
import {
  Journal,
  type JournalCommand,
  type JournalQuery,
  type JournalService,
} from "@white-rabbit/frontend-core/services/journal";
import type { H3Event } from "h3";

const convertFromProto = (proto: JournalProto): Journal => {
  return new Journal(
    proto.id,
    proto.createdDate ?? "",
    proto.version,
    proto.name,
    proto.description,
    proto.unit,
    proto.tags,
  );
};

class JournalClientImpl implements JournalService {
  private grpcClient: JournalServiceClient;

  constructor(apiBase: string) {
    this.grpcClient = new JournalServiceClient(apiBase, ChannelCredentials.createInsecure());
  }

  async findById(id: string): Promise<[Journal, Map<string, ReadAggregate>] | undefined> {
    const resp: JournalProto | undefined = await new Promise((resolve, reject) => {
      this.grpcClient.findById(FindByIdRequest.fromPartial({ id }), (err, resp) => {
        if (err) {
          reject(err);
        } else {
          resolve(resp.value);
        }
      });
    });

    if (resp) {
      return [convertFromProto(resp), new Map()];
    } else {
      return undefined;
    }
  }

  async findAll(query: JournalQuery): Promise<[Journal[], Map<string, ReadAggregate>]> {
    const resp: JournalProto[] = await new Promise((resolve, reject) => {
      this.grpcClient.findAll(FindAllRequest.fromPartial({ query }), (err, resp) => {
        if (err) {
          reject(err);
        } else {
          resolve(resp.values);
        }
      });
    });

    return [resp.map((item) => convertFromProto(item)), new Map()];
  }

  async handleCommand(command: JournalCommand): Promise<string[]> {
    const resp: string[] = await new Promise((resolve, reject) => {
      this.grpcClient.handleCommand(
        HandleCommandRequest.fromPartial({ command: command }),
        (err, resp) => {
          if (err) {
            reject(err);
          } else {
            resolve(resp.values);
          }
        },
      );
    });

    return resp;
  }
}

let journalClient: JournalClientImpl;

export const useJournalClient = (event: H3Event): JournalService => {
  if (!journalClient) {
    const {
      public: { apiBase },
    } = useRuntimeConfig(event);
    journalClient = new JournalClientImpl(apiBase);
  }

  return journalClient;
};
