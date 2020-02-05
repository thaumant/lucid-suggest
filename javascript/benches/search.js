const {Suite} = require('benchmark')
const {search, storeRecords} = require('../build/index')
const {
    STATES, 
    QUERIES_SINGLE_CHAR,
    QUERIES_PARTIAL,
    QUERIES_FULL,
} = require('./constants')

storeRecords(STATES)

let i1 = 0
let i2 = 0
let i3 = 0

new Suite()
    .add('store', () => {
        storeRecords(STATES)
    })
    .add('queries_single_char', () => {
        const query = QUERIES_SINGLE_CHAR[i1++ % QUERIES_SINGLE_CHAR.length]
        search(query)
    })
    .add('queries_partial', () => {
        const query = QUERIES_PARTIAL[i2++ % QUERIES_PARTIAL.length]
        search(query)
    })
    .add('queries_full', () => {
        const query = QUERIES_FULL[i3++ % QUERIES_FULL.length]
        search(query)
    })
    .on('complete', function () {
        const results = []
        this.forEach(bench => {
            const name = bench.name
            const mean = Math.round(bench.stats.mean * 1e6) + ' μs'
            results.push({name, mean})
        })
        console.table(results)
    })
    .run()
