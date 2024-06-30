import { definePreset, palette } from "@primevue/themes";
import Aura from "@primevue/themes/aura";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const preset = definePreset(Aura, {
  semantic: {
    primary: palette("{green}"),
  },
});

const currentDir = dirname(fileURLToPath(import.meta.url));

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@primevue/nuxt-module", "@nuxtjs/tailwindcss", "@nuxt/eslint"],
  compatibilityDate: "2024-07-03",
  devtools: {
    enabled: true,
    timeline: {
      enabled: true,
    },
  },
  css: [join(currentDir, "./assets/scss/main.scss")],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  primevue: {
    options: {
      theme: {
        preset: preset,
        options: {
          cssLayer: {
            name: "primevue",
            order: "tailwind-base, primevue, tailwind-utilities",
          },
        },
      },
    },
  },
});
