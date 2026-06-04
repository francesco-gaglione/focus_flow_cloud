import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
import type { UserConfig } from 'vite'

const isTauri = process.env.TAURI_ENV_PLATFORM !== undefined

const config: UserConfig = {
  plugins: [
    sveltekit(),
    tailwindcss(),
  ],
  // Tauri needs a fixed port and no host exposure
  server: {
    port: 5173,
    strictPort: true,
    hmr: { port: 24678 },
    proxy: isTauri ? undefined : {
      '/api': {
        target: process.env.VITE_API_BASE_URL || 'http://localhost:8080',
        changeOrigin: true,
      },
      '/ws': {
        target: process.env.VITE_API_BASE_URL || 'http://localhost:8080',
        changeOrigin: true,
        ws: true,
      },
    },
  },
  // Tauri expects a consistent dev server URL
  clearScreen: false,
}

export default config
