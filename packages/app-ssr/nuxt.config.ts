// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  extends: ["@white-rabbit/frontend-core"],
  runtimeConfig: {
    apiSecret: "local_secret",
    public: {
      apiBase: "http://localhost:8080",
    },
  },
});
