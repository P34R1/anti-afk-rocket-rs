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
                dir: resolve(__dirname, "templates"), // Adjust the output directory as needed
            },
        },
    },
    resolve: {
        alias: {
            // "@": fileURLToPath(new URL("./src/", import.meta.url)),
            "@": resolve("./frontend/"),
        },
    },
})
// export default defineConfig({
//     build: {
//         rollupOptions: {
//             input: {
//                 main: resolve(__dirname, "index.html"), // Adjust the path as needed
//                 // main: resolve(__dirname, "src", "index.ts"), // Adjust the path as needed
//             },
//             output: {
//                 dir: resolve(__dirname, "templates"), // Adjust the output directory as needed
//                 format: "iife",
//                 entryFileNames: "index.js",
//             },
//         },
//     },
//     resolve: {
//         alias: {
//             // "@": fileURLToPath(new URL("./src/", import.meta.url)),
//             "@": resolve("./src-frontend/"),
//         },
//     },
// })
