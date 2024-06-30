// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  extends: ["@white-rabbit/frontend-core"],
  ssr: false,
  telemetry: false,
  devServer: {
    port: 1420,
    host: "0.0.0.0",
  },
  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
      hmr: {
        // Use websocket for mobile hot reloading
        protocol: "ws",
        // Make sure it's available on the network
        host: "0.0.0.0",
        // Use a specific port for hmr
        port: 5183,
      },
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  },
});
