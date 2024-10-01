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
        "primary-light": "#e3f2fd",
        "primary-main": "#90caf9",
        "primary-dark": "#42a5f5",
        "secondary-light": "#f3e5f5",
        "secondary-main": "#ce93d8",
        "secondary-dark": "#ab47bc",
        "error-light": "#e57373",
        "error-main": "#f44336",
        "error-dark": "#d32f2f",
        "warning-light": "#ffb74d",
        "warning-main": "#ffa726",
        "warning-dark": "#f57c00",
        "info-light": "#4fc3f7",
        "info-main": "#29b6f6",
        "info-dark": "#0288d1",
        "success-light": "#81c784",
        "success-main": "#66bb6a",
        "success-dark": "#388e3c",
      }
    },
  },
  plugins: [
    require("flowbite/plugin"),
  ],
};
