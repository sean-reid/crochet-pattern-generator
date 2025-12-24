/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Primary colors
        white: '#FFFFFF',
        'off-white': '#FAFAFA',
        'charcoal': '#1A1A1A',
        'gray-medium': '#6B6B6B',
        'gray-pale': '#E8E8E8',
        'gray-light': '#F5F5F5',
        
        // Accent colors
        'terracotta': {
          DEFAULT: '#C67B5C',
          dark: '#B56A4D',
          darker: '#A45A3D',
        },
        'sage': '#8BA888',
        'coral': {
          DEFAULT: '#E89B87',
          light: '#FFEEE9',
        },
        'burgundy': '#8B4F4F',
        
        // Functional colors
        'blue-pale': '#E3F2FD',
        'green-light': '#E8F5E9',
        'amber-soft': '#FFF9E6',
        'rose-pale': '#FFEBEE',
        
        // 3D Viewer
        'viewer-bg': '#2A2A2A',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['IBM Plex Mono', 'JetBrains Mono', 'Consolas', 'monospace'],
      },
      fontSize: {
        'xs': ['11px', '1.4'],
        'sm': ['13px', '1.5'],
        'base': ['15px', '1.6'],
        'lg': ['18px', '1.4'],
        'xl': ['24px', '1.3'],
        '2xl': ['32px', '1.2'],
      },
      spacing: {
        '4': '4px',
        '8': '8px',
        '12': '12px',
        '16': '16px',
        '24': '24px',
        '32': '32px',
        '40': '40px',
        '48': '48px',
        '64': '64px',
      },
      borderRadius: {
        'sm': '4px',
        DEFAULT: '6px',
        'md': '8px',
        'lg': '12px',
      },
      boxShadow: {
        'sm': '0 1px 3px rgba(0, 0, 0, 0.06)',
        DEFAULT: '0 2px 8px rgba(0, 0, 0, 0.1)',
        'md': '0 4px 12px rgba(0, 0, 0, 0.08)',
        'lg': '0 4px 12px rgba(0, 0, 0, 0.15)',
        'xl': '0 20px 60px rgba(0, 0, 0, 0.3)',
        'inner-sm': 'inset 0 2px 4px rgba(0, 0, 0, 0.1)',
        'inner': 'inset 0 2px 8px rgba(0, 0, 0, 0.3)',
      },
      transitionDuration: {
        '150': '150ms',
        '200': '200ms',
        '250': '250ms',
        '300': '300ms',
        '400': '400ms',
      },
      transitionTimingFunction: {
        'out': 'cubic-bezier(0.4, 0, 0.2, 1)',
        'in': 'cubic-bezier(0.4, 0, 1, 1)',
        'in-out': 'cubic-bezier(0.4, 0, 0.2, 1)',
      },
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        'slide-up': {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        'scale-in': {
          '0%': { transform: 'scale(0.95)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
        'shimmer': {
          '0%': { transform: 'translateX(-100%)' },
          '100%': { transform: 'translateX(100%)' },
        },
      },
      animation: {
        'fade-in': 'fade-in 300ms ease-out',
        'slide-up': 'slide-up 300ms ease-out',
        'scale-in': 'scale-in 250ms ease-out',
        'shimmer': 'shimmer 2s infinite',
      },
    },
  },
  plugins: [],
}
