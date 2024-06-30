import {
  HelloRequest,
  HelloServiceClient,
} from "../proto/gen/whiterabbit/helloworld/v1/helloworld";
import { ChannelCredentials } from "@grpc/grpc-js";

class HelloClientImpl {
  private grpcClient: HelloServiceClient;

  constructor(apiBase: string) {
    this.grpcClient = new HelloServiceClient(apiBase, ChannelCredentials.createInsecure());
  }

  async hello(name?: string): Promise<object> {
    return new Promise((resolve, reject) => {
      this.grpcClient.hello(HelloRequest.fromPartial({ name: name }), (err, resp) => {
        if (err) {
          reject(err);
        } else {
          resolve(resp);
        }
      });
    });
  }
}

let helloClient: HelloClientImpl;

export default defineEventHandler(async (event) => {
  const { name } = getQuery(event);

  if (!helloClient) {
    const {
      public: { apiBase },
    } = useRuntimeConfig(event);
    helloClient = new HelloClientImpl(apiBase);
  }

  return helloClient.hello(name?.toString());
});
