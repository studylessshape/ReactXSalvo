import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { Console } from 'node:console'
import process from 'node:process';
import { opendir } from 'node:fs/promises';
import { Dirent } from 'node:fs';

const __dirname = dirname(fileURLToPath(import.meta.url))
const console = new Console({ stdout: process.stdout, stderr: process.stderr });
const exludeDir = ['public', 'node_modules'];

// let rollupOptionsInput= {};

// async function findIndexHtml(dir: Dirent): Promise<Dirent[] | null> {
//   if (dir.name === 'index.html' && dir.isFile()) {
//     return [dir];
//   } else if (dir.isDirectory()) {
//     const files = await opendir(dir.path);
//     var indexPath: Dirent[] = [];
//     for await (const dirent of files) {
//       const path = await findIndexHtml(dirent);
//       if (path != null) {
//         indexPath = indexPath.concat(path);
//       }
//     }
//     return indexPath.length > 0 ? indexPath : null;
//   }
//   return null;
// }

// try {
//   const dir = await opendir(__dirname);
//   for await (const dirent of dir) {
//     console.log(dirent);
//     if (exludeDir.find((e) => e == dirent.name)) {
//       continue;
//     }
//   }
// } catch (err) {
//   console.error(err);
// } 

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
  },
  build:{
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        pages: resolve(__dirname, 'pages/index.html'),
      }
    }
  }
})
