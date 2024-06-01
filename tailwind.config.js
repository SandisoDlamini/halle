/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/*.html"],
  theme: {
    extend: {
      animation: {
        "bounce-slow": "bounce 8s infinite",
      },
    },
  },
  plugins: [],
};
