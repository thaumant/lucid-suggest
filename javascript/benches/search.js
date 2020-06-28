const {Suite} = require('benchmark')
const {LucidSuggest} = require('../en')
const {generateRecords, generateQueries} = require('./dataset')


const SAMPLE_SIZE     = 100
const MIN_WORDS       = 3
const MAX_WORDS       = 5
const RECORDS         = generateRecords(SAMPLE_SIZE, MIN_WORDS, MAX_WORDS)
const QUERIES_1CHAR   = generateQueries(1000, RECORDS, 0, 0)
const QUERIES_PARTIAL = generateQueries(1000, RECORDS, 0, 1)
const QUERIES_FULL    = generateQueries(1000, RECORDS, 1, 1)


const SUGGEST = new LucidSuggest()
SUGGEST.setRecords(RECORDS)


let offset1 = 0
let offset2 = 0
let offset3 = 0

new Suite()
    .add({
        name: 'queries_single_char',
        defer: true,
        async fn(deferred) {
            const query = QUERIES_1CHAR[offset1++ % QUERIES_1CHAR.length]
            await SUGGEST.search(query)
            deferred.resolve()
        },
    })
    .add({
        name: 'queries_partial',
        defer: true,
        async fn(deferred) {
            const query = QUERIES_PARTIAL[offset2++ % QUERIES_PARTIAL.length]
            await SUGGEST.search(query)
            deferred.resolve()
        },
    })
    .add({
        name: 'queries_full',
        defer: true,
        async fn(deferred) {
            const query = QUERIES_FULL[offset3++ % QUERIES_FULL.length]
            await SUGGEST.search(query)
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
