import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import unocss from 'unocss/vite'
import { extractorSvelte } from '@unocss/core'
import { presetWind } from "unocss"
import presetIcons from "@unocss/preset-icons"

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [unocss({
      extractors: [extractorSvelte],
      presets: [presetIcons(), presetWind()],
      shortcuts: {
        "btn": "hover:bg-gray-500/50 active:bg-opacity-75",
        "table-header": "bg-dplime text-black"
      },
      theme: {
        colors: {
          dplime: {
            DEFAULT: "#c5ff00",
            dark: "#9ccc00"
          },
        },
      },
    }), svelte()],
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
