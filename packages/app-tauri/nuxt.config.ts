import { internalIpV4 } from "internal-ip";

const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM as string);

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  extends: ["@white-rabbit/frontend-core"],
  devServer: {
    port: 1420,
    host: mobile ? "0.0.0.0" : undefined,
  },
  vite: {
    clearScreen: false,
    server: {
      strictPort: true,
      hmr: mobile
        ? {
            protocol: "ws",
            host: await internalIpV4(),
            port: 1421,
          }
        : undefined,
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  },
});
