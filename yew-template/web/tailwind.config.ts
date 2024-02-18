import type { Config } from 'tailwindcss'

export default {
  content: [
    "./index.html",
    "./src/**/*.{ts,js}",
    "../app/src/**/*.rs"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
} satisfies Config

