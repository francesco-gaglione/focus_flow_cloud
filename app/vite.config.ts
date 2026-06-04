import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
import type { UserConfig } from 'vite'

const config: UserConfig = {
  plugins: [
    sveltekit(),
    tailwindcss(),
  ],
  server: {
    port: 5173,
    strictPort: true,
    hmr: { port: 24678 },
  },
  clearScreen: false,
}

export default config
