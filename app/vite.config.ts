import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
import type { UserConfig } from 'vite'
import { readFileSync } from 'fs'

const pkg = JSON.parse(readFileSync(new URL('./package.json', import.meta.url), 'utf-8'))

const config: UserConfig = {
  define: {
    __APP_VERSION__: JSON.stringify(pkg.version),
  },
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
