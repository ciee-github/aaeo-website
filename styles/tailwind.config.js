const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

module.exports = {
    content: [
        "./src/**/*.{rs,html,css}",
        "./styles/**/*.css",
    ],
    theme: {
        screens: {
            "2xs": "370px",
            xs: "475px",
            ...defaultTheme.screens,
            "3xl": "1792px",
        },
    },
    variants: {},
};
