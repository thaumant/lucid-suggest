import fs from 'fs'

export default () => ({
    name: 'wasm',

    load(id) {
        return /\.wasm$/.test(id)
            ? fs.promises.readFile(id).then(b => b.toString('binary'))
            : null
    },

    transform(code, id) {
        if (code && /\.wasm$/.test(id)) {
            const src = Buffer.from(code, 'binary').toString('base64');
            return `
                var src = '${src}'

                var buf = null
                var isNode = typeof process !== 'undefined' && process.versions != null && process.versions.node != null
                if (isNode) {
                    buf = Buffer.from(src, 'base64')
                } else {
                    var raw = window.atob(src)
                    buf = new Uint8Array(new ArrayBuffer(raw.length))
                    for(var i = 0; i < raw.length; i++) {
                        buf[i] = raw.charCodeAt(i)
                    }
                }

                var module   = new WebAssembly.Module(buf)
                var memory   = new WebAssembly.Memory({initial: 10})
                var instance = new WebAssembly.Instance(module, {env: {memory}})

                export default instance.exports
            `;
        }
    },
})