import cjs from './config-cjs'

export default {
    ...cjs,
    output: {
        file: 'dist/bundle.js',
        format: 'iife',
        name: 'lucidSuggest'
    },
}
