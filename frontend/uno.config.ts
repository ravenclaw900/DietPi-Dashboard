import { defineConfig } from "unocss";
import { presetWind, presetIcons } from "unocss";

export default defineConfig({
  presets: [
    presetIcons({
      collections: {
        // Load icons on-demand
        fa: () => import("@iconify-json/fa6-solid/icons.json").then(i => i.default),
      },
    }),
    presetWind(),
  ],
  shortcuts: {
    btn: "hover:bg-gray-500 hover:bg-opacity-50 active:bg-opacity-75",
    "table-header": "bg-dplime text-black",
  },
  theme: {
    colors: {
      dplime: {
        DEFAULT: "#c5ff00",
        dark: "#9ccc00",
      },
    },
  },
});
