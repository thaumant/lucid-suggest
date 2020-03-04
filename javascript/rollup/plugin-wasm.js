import fs from 'fs'

export default () => ({
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
})