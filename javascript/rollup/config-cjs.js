import wasm from './plugin-wasm'
import pkg  from './plugin-pkg-es6'

export default {
    input: 'src/index.js',
    output: {
        file: 'build/index.js',
        format: 'cjs',
    },
    plugins: [
        wasm(),
        pkg(),
    ]
}