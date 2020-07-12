const {Suite} = require('benchmark')
const {performance} = require('perf_hooks')
const {LucidSuggest} = require('../en')
const {first2digits} = require('./utils')
const {generateRecords, generateQueries} = require('./dataset')

const PRESETS = {
    '2-4 words': {minWords: 2, maxWords: 4, queryMaxLen: 1.0},
    '4-8 words': {minWords: 4, maxWords: 8, queryMaxLen: 0.5},
}

run()

async function run() {
    const suite = new Suite()

    const INDEXING = {}
    const SEARCH   = {}

    for (const nWords in PRESETS) {
        for (const nRecords of [100, 1000, 10000, 100000]) {
            const preset     = PRESETS[nWords]
            const records    = generateRecords(nRecords, preset.minWords, preset.maxWords)
            const queries    = generateQueries(10000, records, 0, preset.queryMaxLen)
            const suggest    = new LucidSuggest()

            await suggest.setupQueue
            const start = performance.now()
            suggest.addRecords(records)
            await suggest.setupQueue
            const end = performance.now()

            INDEXING[`${nRecords} records`] = INDEXING[`${nRecords} records`] || {}
            INDEXING[`${nRecords} records`][nWords] = first2digits(end - start) + " ms"

            let offset = 0
            suite.add({
                name: `${nWords}, ${nRecords} records`,
                defer: true,
                async fn(deferred) {
                    await suggest.search(queries[offset])
                    offset = (offset + 1) % queries.length
                    deferred.resolve()
                },
            })
        }
    }

    console.log("Indexing:")
    console.table(INDEXING)
    console.log("\n")

    suite.on('complete', function () {
        this.forEach(({name: benchName, stats: {mean}}) => {
            const [nWords, nRecords] = benchName.split(', ')
            SEARCH[nRecords]         = SEARCH[nRecords] || {}
            SEARCH[nRecords][nWords] = first2digits(mean * 1e6) + ' μs'
        })
        console.log("Search:")
        console.table(SEARCH)
    })

    suite.run()
}

