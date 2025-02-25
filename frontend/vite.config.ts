import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5702,
    proxy: {
      '/api': {
        target: 'http://localhost:5701',
        changeOrigin: true,
      },
      '/wwwroot': {
        target: 'http://localhost:5701',
        changeOrigin: true,
      },
      '/static': {
        target: 'http://localhost:5701',
        changeOrigin: true,
      },
      '/ws': {
        target: 'http://localhost:5701',
        changeOrigin: true,
        ws: true,
      }
    }
  }
})
