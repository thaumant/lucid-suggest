import * as fs from 'fs'


export default {
    input: "tmp/lucid_suggest_wasm.js",
    output: {
        file: `src/wasm.js`,
        format: "es",
    },
    plugins: [
        pluginWasm(),
        pluginBindgenAsync(),
    ]
}


function pluginBindgenAsync() {
    return {
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

                export const compileWasm = exportsPromise.then(function(wasm) {
                    const __exports__ = {};

                    ${code}

                    return __exports__;
                });
                `
                return code
            }
        },
    }
}


function pluginWasm() {
    return {
        name: 'wasm',
        load(path) {
            if (/\.wasm$/.test(path)) {
                return fs.promises.readFile(path).then(b => b.toString('binary'))
            }
        },
        transform(code, path) {
            if (code && /\.wasm$/.test(path)) {
                const src = Buffer.from(code, 'binary').toString('base64');
                return `
                    var src = '${src}'

                    var binary = null

                    var isNode = typeof process !== 'undefined'
                        && process.versions != null
                        && process.versions.node != null

                    if (isNode) {
                        binary = Buffer.from(src, 'base64')
                    } else {
                        var raw = window.atob(src)
                        binary = new Uint8Array(new ArrayBuffer(raw.length))
                        for(var i = 0; i < raw.length; i++) {
                            binary[i] = raw.charCodeAt(i)
                        }
                    }

                    var instantiatePromise = WebAssembly.instantiate(
                        binary,
                        {env: {memory: new WebAssembly.Memory({initial: 10})}}
                    )

                    var exportsPromise = instantiatePromise.then(result => {
                        return result.instance.exports
                    })

                    export default exportsPromise
                `;
            }
        },
    }
}
