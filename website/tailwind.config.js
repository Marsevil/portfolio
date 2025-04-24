/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,svelte,js,ts}'],
  theme: {
    extend: {
      colors: {
        "professional": "var(--professional)",
        "education": "var(--education)"
      }
    },
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: [
      {
        custom: {
          // Color from forest theme
          "base-100": "oklch(20.84% 0.008 17.911)",
          "base-200": "oklch(18.522% 0.007 17.911)",
          "base-300": "oklch(16.203% 0.007 17.911)",
          "base-content": "oklch(83.768% 0.001 17.911)",
          "primary": "oklch(68.628% 0.185 148.958)",
          "primary-content": "oklch(0% 0 0)",
          "secondary": "oklch(69.776% 0.135 168.327)",
          "secondary-content": "oklch(13.955% 0.027 168.327)",
          "accent": "oklch(70.628% 0.119 185.713)",
          "accent-content": "oklch(14.125% 0.023 185.713)",
          "neutral": "oklch(30.698% 0.039 171.364)",
          "neutral-content": "oklch(86.139% 0.007 171.364)",
          "info": "oklch(72.06% 0.191 231.6)",
          "info-content": "oklch(0% 0 0)",
          "success": "oklch(64.8% 0.15 160)",
          "success-content": "oklch(0% 0 0)",
          "warning": "oklch(84.71% 0.199 83.87)",
          "warning-content": "oklch(0% 0 0)",
          "error": "oklch(71.76% 0.221 22.18)",
          "error-content": "oklch(0% 0 0)",

          "--professional": "oklch(70% 0.14 182.503)",
          "--education": "oklch(93% 0.127 124.321)",

          // Shapes from sunset theme
          "--radius-selector": "1rem",
          "--radius-field": "0.5rem",
          "--radius-box": "1rem",
          "--size-selector": "0.25rem",
          "--size-field": "0.25rem",
          "--border": "1px",
          "--depth": "0",
          "--noise": "0",
        }
      }
    ]
  }
}

