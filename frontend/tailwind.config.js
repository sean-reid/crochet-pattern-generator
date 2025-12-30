/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        cream: {
          50: '#FEFDFB',
          100: '#FBF9F5',
          200: '#F7F3EB',
          300: '#F0EBE0',
          400: '#E8E1D4',
          500: '#DDD5C7',
        },
        terracotta: {
          400: '#D97757',
          500: '#C8603F',
          600: '#B54D2E',
        },
        sage: {
          500: '#7A9B76',
          600: '#658065',
        },
        clay: {
          500: '#C85A4F',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      borderRadius: {
        'xl': '12px',
        '2xl': '16px',
      },
    },
  },
  plugins: [],
}
