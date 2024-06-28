import type { Config } from "tailwindcss";
// @ts-expect-error no declared types at this time
import primeui from "tailwindcss-primeui";
import typography from "@tailwindcss/typography";

export default {
  corePlugins: {
    preflight: false,
  },
  plugins: [primeui, typography],
} satisfies Partial<Config>;
