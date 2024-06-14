import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import unocss from "unocss/vite";
import extractorSvelte from "@unocss/extractor-svelte";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [
      unocss({
        extractors: [extractorSvelte()],
      }),
      svelte(),
    ],
    build: {
      // https://github.com/vitejs/vite/commit/74fa024
      manifest: "manifest.json",
      rollupOptions: {
        input: "src/main.ts",
        output: {
          manualChunks: {
            xterm: ["xterm", "xterm-addon-attach", "xterm-addon-fit"],
          },
        },
      },
    },
    define: {
      __PACKAGE_VERSION__: JSON.stringify(process.env.npm_package_version),
    },
  };
});
