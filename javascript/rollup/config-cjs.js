import wasm         from './plugin-wasm'
import bindgenAsync from './plugin-bindgen-async'

const lang = (process.env.RUSTFLAGS || '').match(/lang="(\w+)"/)?.[1]

export default {
    input: 'src/index.js',
    output: {
        file: lang ? `dist/${lang}.js` : 'dist/index.js',
        format: 'cjs',
    },
    plugins: [
        wasm(),
        bindgenAsync(),
    ]
}
