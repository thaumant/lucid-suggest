export default ({lang}) => ({
    name: 'bindgen-async',

    transform(code, path) {
        if (/\/build\/index\.js$/.test(path)) {
            code = code.replace(/lang\/lang-placeholder/, `lang/lang-${lang}`)
            code = code.replace(/\.\/wasm-placeholder/, `../pkg/lucid_suggest_wasm`)
            return code
        }
    },
})