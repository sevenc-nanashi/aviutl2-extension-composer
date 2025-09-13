import eslint from "@eslint/js";
import { defineConfig } from "eslint/config";
import gitignore from "eslint-config-flat-gitignore";
import prettierConfig from "eslint-config-prettier";
import progress from "eslint-plugin-file-progress";
import importPlugin from "eslint-plugin-import";
import prettierPlugin from "eslint-plugin-prettier/recommended";
import eslintPluginVue from "eslint-plugin-vue";
import globals from "globals";
import tseslint from "typescript-eslint";

export default defineConfig(
  gitignore(),
  progress.configs.recommended,
  prettierPlugin,
  {
    extends: [
      eslint.configs.recommended,
      ...tseslint.configs.recommended,
      ...eslintPluginVue.configs["flat/recommended"],
      importPlugin.flatConfigs.recommended,
      importPlugin.flatConfigs.typescript,
    ],
    files: ["**/*.{ts,vue}"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: globals.browser,
      parserOptions: {
        parser: tseslint.parser,
      },
    },
    rules: {
      // false positiveが多すぎるので無効化
      "import/no-unresolved": "off",
      "import/order": [
        "error",
        {
          alphabetize: { order: "asc", caseInsensitive: true },
        },
      ],
      "import/extensions": [
        "error",
        {
          js: "never",
          ts: "always",
          vue: "always",
          yml: "always",
        },
      ],
      "import/enforce-node-protocol-usage": ["error", "always"],

      "vue/multi-word-component-names": "off",
    },
  },
  prettierConfig,
  {
    files: ["./src/lib/models/*.d.ts"],
    linterOptions: {
      reportUnusedDisableDirectives: "off",
    },
  },
);
