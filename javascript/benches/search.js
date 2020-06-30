const {Suite} = require('benchmark')
const {LucidSuggest} = require('../en')
const {generateRecords, generateQueries} = require('./dataset')

const PRESETS = [
    {minWords: 2, maxWords: 4, queryMaxLen: 1.0},
    {minWords: 4, maxWords: 8, queryMaxLen: 0.5},
]

const SAMPLE_SIZE = 100
const PRESET      = PRESETS[0]
const RECORDS     = generateRecords(SAMPLE_SIZE, PRESET.minWords, PRESET.maxWords)
const QUERIES     = generateQueries(10000, RECORDS, 0, PRESET.queryMaxLen)

const SUGGEST = new LucidSuggest()
SUGGEST.setRecords(RECORDS)

let offset = 0

new Suite()
    .add({
        name: 'queries_partial',
        defer: true,
        async fn(deferred) {
            await SUGGEST.search(QUERIES[offset])
            offset = (offset + 1) % QUERIES.length
            deferred.resolve()
        },
    })
    .on('complete', function () {
        console.log()
        this.forEach(({name, hz, stats: {mean}}) => {
            console.log(name)
            console.log(`    ${(mean * 1e6).toFixed(2)} μs`)
            console.log(`    ${Math.round(hz)} ops/s`)
            console.log()
        })
    })
    .run()
