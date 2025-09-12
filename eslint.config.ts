import { defineConfig } from "eslint/config";
import eslint from "@eslint/js";
import prettierConfig from "eslint-config-prettier";
import eslintPluginVue from "eslint-plugin-vue";
import globals from "globals";
import tseslint from "typescript-eslint";
import progress from "eslint-plugin-file-progress";
import gitignore from "eslint-config-flat-gitignore";
import prettierPlugin from "eslint-plugin-prettier/recommended";
import importPlugin from "eslint-plugin-import";

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
      "import/order": "error",
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
