import wasm         from './plugin-wasm'
import bindgenAsync from './plugin-bindgen-async'

const lang = (process.env.RUSTFLAGS || '').match(/lang="(\w+)"/)?.[1]

if (!lang) {
    throw new Error("Missing RUSTFLAGS=\"--cfg lang=\"*\"\"")
}

export default {
    input: 'src/index.js',
    output: {
        file: `${lang}.js`,
        format: 'cjs',
    },
    plugins: [
        wasm(),
        bindgenAsync(),
    ]
}
