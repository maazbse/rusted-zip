import { defineConfig } from "vite";

export default defineConfig({
  base: '/rusted-zip/',
  server: {
    fs: {
      allow: [".."],
    },
  },
});