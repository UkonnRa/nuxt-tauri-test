// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  extends: ["@white-rabbit/frontend-core"],
  runtimeConfig: {
    apiSecret: "local_secret",
    public: {
      apiBase: "[::1]:50051",
    },
  },
  alias: {
    // The original import "protobufjs/minimal" is a CJS format and NOT suitable with the ESM one, need to import the orignal JS
    // https://github.com/protobufjs/protobuf.js/issues/1929
    "protobufjs/minimal": "protobufjs/minimal.js",
  },
});
