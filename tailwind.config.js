/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        "background": "hsl(var(--color-background) / <alpha-value>)",
        "primary-text": "hsl(var(--color-primary-text) / <alpha-value>)",
        "secondary-text": "hsl(var(--color-primary-text) / <alpha-value>)",
        "accent": "hsl(var(--color-accent) / <alpha-value>)",
        "highlight": "hsl(var(--color-highlight) / <alpha-value>)",
        // "static-black": "hsl(var(--color-static-black) / <alpha-value>)",
        // "static-white": "hsl(var(--color-static-white) / <alpha-value>)",
        // "switch-text": "hsl(var(--color-switch-text) / <alpha-value>)"
      },
    },
  },
  plugins: [],
}
