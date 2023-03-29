import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import unocss from 'unocss/vite'
import { presetWind, presetIcons, transformerDirectives } from "unocss"

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [unocss({
      presets: [presetIcons(), presetWind()],
      transformers: [transformerDirectives()],
      shortcuts: {
        "btn": "hover:bg-gray-500 hover:bg-opacity-50 active:bg-opacity-75",
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
      mode: "svelte-scoped",
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
