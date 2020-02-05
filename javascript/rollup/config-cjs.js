import wasm from './plugin-wasm'

export default {
    input: 'src/index.js',
    output: {
        file: 'build/index.js',
        format: 'cjs',
    },
    plugins: [
        wasm(),
    ]
}