import js from "@eslint/js";
import pluginVue from "eslint-plugin-vue";
import tseslint from "typescript-eslint";
import vueEslintParser from "vue-eslint-parser";
import globals from "globals";
import eslintConfigPrettier from "eslint-config-prettier";

/**
 * @type {import('@types/eslint').FlatConfig[]}
 */
export default [
  js.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  eslintConfigPrettier,
  {
    ignores: [
      "coverage/",
      "target/",
      "**/.nuxt/",
      "**/.output/",
      "packages/app-tauri/src-tauri/gen/",
      "packages/app-tauri/src-tauri/capabilities/",
    ],
  },
  {
    languageOptions: {
      parser: tseslint.parser,
      globals: {
        ...globals.browser,
        ...globals.es2021,
        ...globals.node,
      },
    },
  },
  {
    files: ["**/*.vue"],
    languageOptions: {
      parser: vueEslintParser,
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },
  {
    rules: {
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
    },
  },
];
