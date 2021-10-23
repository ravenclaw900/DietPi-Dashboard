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
})