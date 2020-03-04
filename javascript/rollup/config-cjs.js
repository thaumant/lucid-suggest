import wasm         from './plugin-wasm'
import bindgenAsync from './plugin-bindgen-async'

export default {
    input: 'src/index.js',
    output: {
        file: 'build/index.js',
        format: 'cjs',
    },
    plugins: [
        wasm(),
        bindgenAsync(),
    ]
}
