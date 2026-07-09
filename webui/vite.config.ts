import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

const controllerTarget = process.env.VITE_CONTROLLER_URL ?? "http://127.0.0.1:9100";
const buildBase = process.env.VITE_BASE ?? "/ui/";

export default defineConfig(({ command }) => ({
  base: command === "build" ? buildBase : "/",
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src")
    }
  },
  server: {
    port: 5173,
    proxy: {
      "/v1": {
        target: controllerTarget,
        ws: true
      },
      "/healthz": controllerTarget,
      "/metrics": controllerTarget
    }
  }
}));
