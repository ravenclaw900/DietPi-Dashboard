const purgecss = require('@fullhuman/postcss-purgecss')({
    content: [
        './src/**/*.html',
        './src/**/*.svelte'
    ],

    whitelistPatterns: [/svelte-/],

    defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
});

module.exports = {
    plugins: [
        require('tailwindcss'),
        ...([purgecss]),
        require("autoprefixer")
    ]
};