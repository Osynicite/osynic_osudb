import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
import { visualizer } from "rollup-plugin-visualizer";
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(),
    tailwindcss(),
    wasm(),
    topLevelAwait(),
    visualizer({
      open: true,
    })]
})
