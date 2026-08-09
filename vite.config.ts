import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import tsConfigPath from "vite-tsconfig-paths";

// https://vite.dev/config/
export default defineConfig({
    plugins: [vue(), tsConfigPath(), tailwindcss(), wasm(), topLevelAwait()],
    build: {
        target: "esnext",
    },
});
