import cjs from './config-cjs'

export default {
    ...cjs,
    output: {
        file: 'build/bundle.js',
        format: 'iife',
        name: 'lucidSuggest'
    },
}
