const {Suite} = require('benchmark')
const LucidSuggest = require('../build/index')
const {
    STATES,
    QUERIES_SINGLE_CHAR,
    QUERIES_PARTIAL,
    QUERIES_FULL,
} = require('./constants')


const suggest = new LucidSuggest()
suggest.setRecords(STATES)

let offset1 = 0
let offset2 = 0
let offset3 = 0

new Suite()
    .add({
        name: 'queries_single_char',
        defer: true,
        async fn(deferred) {
            const query = QUERIES_SINGLE_CHAR[offset1++ % QUERIES_SINGLE_CHAR.length]
            await suggest.search(query)
            deferred.resolve()
        },
    })
    .add({
        name: 'queries_partial',
        defer: true,
        async fn(deferred) {
            const query = QUERIES_PARTIAL[offset2++ % QUERIES_PARTIAL.length]
            await suggest.search(query)
            deferred.resolve()
        },
    })
    .add({
        name: 'queries_full',
        defer: true,
        async fn(deferred) {
            const query = QUERIES_FULL[offset3++ % QUERIES_FULL.length]
            await suggest.search(query)
            deferred.resolve()
        },
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

