import { defineConfig } from 'windicss/helpers'

export default defineConfig({
    darkMode: 'media',
    theme: {
        extend: {
            colors: {
                dplime: {
                    DEFAULT: "#c5ff00",
                    dark: "9ccc00"
                }
            },
        },
    },
    shortcuts: {
        "btn": "hover:bg-gray-500/50 active:bg-opacity-75"
    },
})