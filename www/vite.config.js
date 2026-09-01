import { defineConfig } from 'vite';
import path from 'path';
// Vite serves pkg/ (wasm) from parent — allow fs access.
// Build: `wasm-pack build --target web` → pkg/ before `npm run dev`.
export default defineConfig({
  server: {
    fs: {
      allow: [path.resolve(__dirname, '..')],
    },
  },
});
