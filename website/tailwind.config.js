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
        "background": "#121212",
        "common-black": "#000",
        "common-white": "#fff",
        "primary-light": "#e3f2fd",
        "primary-main": "#90caf9",
        "primary-dark": "#42a5f5",
        "primary-contrast-text": "#000",
        "secondary-light": "#f3e5f5",
        "secondary-main": "#ce93d8",
        "secondary-dark": "#ab47bc",
        "secondary-contrast-text": "#000",
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
        "text-primary": "#fff",
        "focus": "#fff",
        "grey": {
          50: "#fafafa",
          100: "#f5f5f5",
          200: "#eeeeee",
          300: "#e0e0e0",
          400: "#bdbdbd",
          500: "#9e9e9e",
          600: "#757575",
          700: "#616161",
          800: "#424242",
          900: "#212121",
          A100: "#f5f5f5",
          A200: "#eeeeee",
          A400: "#bdbdbd",
          A700: "#616161",
        }
      }
    },
  },
  plugins: [
    require("flowbite/plugin"),
  ],
};
