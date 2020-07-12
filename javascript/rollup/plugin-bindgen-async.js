import fs from 'fs'

export default () => ({
    name: 'bindgen-async',

    load(path) {
        if (/lucid_suggest_wasm\.js$/.test(path)) {
            return fs.promises.readFile(path, 'utf8')
        }
    },

    transform(code, path) {
        if (code && /lucid_suggest_wasm\.js$/.test(path)) {
            code = code.replace(/^import \* as wasm.*\n/, '')
            code = code.replace(/export function ([\w_\d]+)/g, '__exports__.$1 = function')
            code = `
            import exportsPromise from './lucid_suggest_wasm_bg.wasm'

            export default exportsPromise.then(function(wasm) {
                const __exports__ = {};

                ${code}

                return __exports__;
            });
            `
            return code
        }
    },
})