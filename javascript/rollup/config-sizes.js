import * as fs from 'fs'
import babel from '@rollup/plugin-babel'
import {nodeResolve} from '@rollup/plugin-node-resolve'


const LANG = process.env.LANG
if (!LANG) {
    throw new Error("Missing LANG env variable")
}


export default {
    input: "index.js",
    output: {
        file: `tmp/${LANG}.js`,
        format: "cjs",
    },
    plugins: [
        nodeResolve(),
        pluginAddLangImport(LANG),
        babel({babelHelpers: 'bundled'}),
    ]
}


function pluginAddLangImport(lang) {
    return {
        name: 'add-lang-import',
        load(path) {
            if (/javascript\/index\.js$/.test(path)) {
                return fs.promises.readFile(path, 'utf8')
            }
        },
        transform(code, path) {
            if (code && /javascript\/index\.js$/.test(path)) {
                const upper = lang.toUpperCase()
                const lower = lang.toLowerCase()
                return `export {LANG_${upper}} from './lang/lang-${lower}';\n` + code
            }
        },
    }
}