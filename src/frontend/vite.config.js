import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { replaceCodePlugin } from "vite-plugin-replace";
import windi from 'vite-plugin-windicss'

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [svelte(), windi({}), replaceCodePlugin({
      replacements: [
        {
          from: /__PACKAGE_VERSION__/g,
          to: process.env.npm_package_version,
        },
        {
          from: /5252/g,
          to: process.env.NODE_ENV === 'production' ? "window.location.port" : "5252",
        },
      ]
    })],
    build: {
      rollupOptions: {
        output: {
          manualChunks: {
            xterm: ['xterm', 'xterm-addon-attach', 'xterm-addon-fit']
          }
        }
      }
    }
  }
})
