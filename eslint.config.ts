import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";
import eslintConfigPrettier from "eslint-config-prettier";

export default tseslint.config(
    // 1. プロジェクト全体で無視するファイル（独立したオブジェクトで定義）
    {
        ignores: [
            "dist/**",
            "node_modules/**",
            ".vite/**",
            "public/**",
            "coverage/**",
            "eslint.config.ts",
            "wasm/**",
        ],
    },

    // 2. 各推奨ルールセットを展開して適用
    js.configs.recommended,
    ...tseslint.configs.recommended,
    ...pluginVue.configs["flat/essential"],

    // 3. 全ファイル共通の設定（グローバル変数・個別のカスタマイズルール）
    {
        files: ["**/*.{js,mjs,cjs,ts,mts,cts,vue}"],
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node,
            },
        },
        rules: {
            // var を禁止して let / const を強制する設定
            "no-var": "error",
            "prefer-const": "error",
        },
    },

    // 4. typescript特有ルール
    {
        files: ["**/*.{ts,mts,cts,vue}"],
        rules: {
            "@typescript-eslint/naming-convention": "error",
        },
    },

    // 5. Vueファイル内でTypeScriptパーサーを使用する設定
    {
        files: ["**/*.vue"],
        languageOptions: {
            parserOptions: {
                parser: tseslint.parser,
            },
        },
    },

    eslintConfigPrettier
);
