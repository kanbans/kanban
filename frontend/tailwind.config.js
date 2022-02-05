const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  mode: 'jit',
  content: ["./index.html", './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {},
    fontFamily: {
      'sans': ['Inter', ...defaultTheme.fontFamily.sans]
    }
  },
  plugins: [],
}
