import {nodeResolve} from "@rollup/plugin-node-resolve"
import placeholders from "./plugin-placeholders"
import wasm         from "./plugin-wasm"
import bindgenAsync from "./plugin-bindgen-async"


const LANG = process.env.LANG
const SUPPORTED_LANGS = ["en", "de", "en", "es", "fr", "pt", "ru"]
if (!SUPPORTED_LANGS.includes(LANG)) {
    throw new Error(`Unknown lang: ${LANG}`)
}


export default {
    input: "build/index.js",
    output: {
        file: `dist/${LANG}.js`,
        format: "cjs",
    },
    plugins: [
        nodeResolve(),
        placeholders({lang: LANG}),
        wasm(),
        bindgenAsync(),
    ]
}
