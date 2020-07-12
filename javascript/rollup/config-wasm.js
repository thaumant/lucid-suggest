import wasm         from "./plugin-wasm"
import bindgenAsync from "./plugin-bindgen-async"


export default {
    input: "tmp/lucid_suggest_wasm.js",
    output: {
        file: `src/wasm.js`,
        format: "cjs",
    },
    plugins: [
        wasm(),
        bindgenAsync(),
    ]
}
