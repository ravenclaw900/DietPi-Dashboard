import { defineConfig } from 'windicss/helpers'

export default defineConfig({
    darkMode: 'media',
    theme: {
        extend: {
            colors: {
                dplime: {
                    DEFAULT: "#9ccc00",
                    dark: "#7B952F"
                }
            },
        },
    },
    shortcuts: {
        "btn": "hover:bg-gray-500/50 active:bg-opacity-75"
    },
})