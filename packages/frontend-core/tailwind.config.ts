import type { Config } from "tailwindcss";
// @ts-expect-error no declared types at this time
import primeui from "tailwindcss-primeui";

export default {
  corePlugins: {
    preflight: false,
  },
  plugins: [primeui],
} satisfies Partial<Config>;
