import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { replaceCodePlugin } from "vite-plugin-replace";
import windi from 'vite-plugin-windicss'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), windi({}), replaceCodePlugin({
    replacements: [
      {
        from: /__PACKAGE_VERSION__/g,
        to: process.env.npm_package_version,
      },
    ]
  })]
})
