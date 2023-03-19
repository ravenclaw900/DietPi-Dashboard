import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import replace from '@rollup/plugin-replace';
import windi from 'vite-plugin-windicss'

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [svelte(), windi({}), replace({
      '__PACKAGE_VERSION__': process.env.npm_package_version,
    })],
    build: {
      manifest: true,
      rollupOptions: {
        input: "src/main.ts",
        output: {
          manualChunks: {
            xterm: ['xterm', 'xterm-addon-attach', 'xterm-addon-fit']
          }
        }
      }
    }
  }
})
