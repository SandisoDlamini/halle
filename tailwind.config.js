/** @type {import('tailwindcss').Config} */

const plugin = require("tailwindcss/plugin");
const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: ["./templates/*.html"],
  darkMode: "selector",
  theme: {
    extend: {
      fontFamily: {
        rubik: ["Rubik-Regular", ...defaultTheme.fontFamily.sans],
        "rubik-semibold": ["Rubik-Semibold", ...defaultTheme.fontFamily.sans],
        "rubik-medium": ["Rubik-Medium", ...defaultTheme.fontFamily.sans],
        "rubik-italic": ["Rubik-Italic", ...defaultTheme.fontFamily.sans],
        "rubik-semibold-italic": [
          "Rubik-SemiboldItalic",
          ...defaultTheme.fontFamily.sans,
        ],
      },
      animation: {
        "bounce-slow": "bounce 8s infinite",
        "pulse-slow": "pulse 8s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "slideshow-fast": "fade 20s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "slideshow-medium": "fade 24s ease-in-out infinite",
        "background-pan": "background-pan 6s linear infinite",
        movie: "movie 20s linear infinite",
        slide: "slide 4s linear infinite",
        "scroll-left": "scroll-left 18s linear infinite",
      },
      keyframes: {
        fade: {
          "0%": {
            opacity: 0,
            filter: "blur(1px)",
          },
          "25%": {
            opacity: 1,
            filter: "blur(0px)",
          },
          "45%": {
            opacity: 0,
          },
          "50%": {
            opacity: 0,
          },
          "100%": {
            opacity: 0,
          },
        },
        "background-pan": {
          from: {
            backgroundPosition: 0,
          },
          to: {
            backgroundPosition: -100,
          },
        },
        movie: {
          "0%": {
            backgroundPosition: "35% 42%",
            backgroundSize: "200%",
          },
          "20%": {
            backgroundPosition: "60% 57%",
            backgroundSize: "200%",
          },
          "20.0001%": {
            backgroundPosition: "60% 85%",
            backgroundSize: "500%",
          },
          "40%": {
            backgroundPosition: "49% 81%",
            backgroundSize: "500%",
          },
          "40.0001%": {
            backgroundPosition: "80% 42%",
            backgroundSize: "300%",
          },
          "60%": {
            backgroundPosition: "84% 33%",
            backgroundSize: "300%",
          },
          "60.0001%": {
            backgroundPosition: "0% 0%",
            backgroundSize: "300%",
          },
          "80%": {
            backgroundPosition: "15% 4%",
            backgroundSize: "300%",
          },
          "80.0001%": {
            backgroundPosition: "89% 7%",
            backgroundSize: "300%",
          },
          "100%": {
            backgroundPosition: "71% 25%",
            backgroundSize: "300%",
          },
        },
        slide: {
          "0%": {
            opacity: 0,
            filter: "blur(5px)",
            transform: "translate(-100%, 0%)",
          },
          "100%": {
            opacity: 1,
            filter: "blur(0px)",
            transform: "translate(0%, 0%)",
          },
        },
        "scroll-left": {
          to: {
            left: "-288px",
          },
        },
      },
      backgroundSize: {
        auto: "auto",
        cover: "cover",
        contain: "contain",
        "50%": "50%",
        "200%": "200%",
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
    require("@tailwindcss/typography"),
  ],
};
