/** @type {import('tailwindcss').Config} */

const plugin = require("tailwindcss/plugin");

module.exports = {
  content: ["./templates/*.html"],
  theme: {
    extend: {
      animation: {
        "bounce-slow": "bounce 8s infinite",
        "pulse-slow": "pulse 8s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "slideshow-fast": "fade 20s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "slideshow-medium": "fade 24s cubic-bezier(0.4, 0, 0.6, 1) infinite",
      },
      keyframes: {
        fade: {
          "0%": {
            opacity: 0,
          },
          "25%": {
            opacity: 1,
          },
          "50%": {
            opacity: 0,
          },
          "100%": {
            opacity: 0,
          },
        },
      },
    },
  },
  plugins: [
    plugin(({ theme, addUtilities }) => {
      const neonUtilities = {};
      const colors = theme("colors");
      for (const color in colors) {
        if (typeof colors[color] === "object") {
          const color1 = colors[color]["500"];
          const color2 = colors[color]["700"];
          neonUtilities[`.neon-${color}`] = {
            boxShadow: `0 0 5px ${color1}, 0 0 10px ${color2}`,
          };
        }
      }
      addUtilities(neonUtilities);
    }),
  ],
};
