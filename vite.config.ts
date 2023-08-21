import { defineConfig } from "vite"
import { resolve } from "path"

import { viteSingleFile } from "vite-plugin-singlefile"
import { ViteMinifyPlugin } from "vite-plugin-minify"

// https://vitejs.dev/config/
/** @type {import('vite').UserConfig} */
export default defineConfig({
    plugins: [viteSingleFile(), ViteMinifyPlugin()],
    build: {
        rollupOptions: {
            output: {
                dir: resolve(__dirname, "templates"),
            },
        },
    },
    resolve: {
        alias: {
            "@": resolve(__dirname, "frontend"),
        },
    },
})
