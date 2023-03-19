import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import windi from 'vite-plugin-windicss'

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [svelte(), windi({})],
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
    },
    define: {
      __PACKAGE_VERSION__: JSON.stringify(process.env.npm_package_version),
    }
  }
})
