import fs from 'fs'

export default () => ({
    name: 'pkg-es6',

    load(path) {
        if (/\/pkg\/.*\.js$/.test(path)) {
            return fs.promises.readFile(path, 'utf8')
        }
    },

    transform(code, path) {
        if (code && /\/pkg\/.*\.js$/.test(path)) {
            return code.replace(/import \* as wasm/, 'import wasm')
        }
    },
})