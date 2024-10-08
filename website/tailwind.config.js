const colors = require("tailwindcss/colors");
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
    // include flowbite js
    "./node_modules/flowbite/**/*.js",
  ],
  theme: {
    extend: {
      colors: {
        "background": colors.gray[900],
        "background-card": colors.gray[800],
        "common-black": "#000",
        "common-white": "#fff",
        "primary-light": colors.blue[500],
        "primary-main": colors.blue[600],
        "primary-dark": colors.blue[700],
        "primary-contrast-text": colors.white,
        "secondary-light": colors.gray[700],
        "secondary-main": colors.gray[800],
        "secondary-dark": colors.gray[900],
        "secondary-contrast-text": colors.white,
        "error-light": "#e57373",
        "error-main": "#f44336",
        "error-dark": "#d32f2f",
        "error-contrast-text": "#fff",
        "warning-light": "#ffb74d",
        "warning-main": "#ffa726",
        "warning-dark": "#f57c00",
        "warning-contrast-text": "#000",
        "info-light": "#4fc3f7",
        "info-main": "#29b6f6",
        "info-dark": "#0288d1",
        "info-contrast-text": "#000",
        "success-light": "#81c784",
        "success-main": "#66bb6a",
        "success-dark": "#388e3c",
        "success-contrast-text": "#000",
        "text-primary": colors.white,
        "text-secondary": colors.gray[300],
        "focus": colors.blue[800],
        "hover": colors.blue[500],
      }
    },
  },
  plugins: [
    require("flowbite/plugin"),
  ],
};
