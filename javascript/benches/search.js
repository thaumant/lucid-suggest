const {Suite} = require('benchmark')
const {LucidSuggest} = require('../en')
const {first2digits} = require('./utils')
const {generateRecords, generateQueries} = require('./dataset')

const PRESETS = {
    '2-4 words': {minWords: 2, maxWords: 4, queryMaxLen: 1.0},
    '4-8 words': {minWords: 4, maxWords: 8, queryMaxLen: 0.5},
}

const suite = new Suite()

for (const name in PRESETS) {
    for (const sampleSize of [100, 1000, 10000]) {
        const preset     = PRESETS[name]
        const records    = generateRecords(sampleSize, preset.minWords, preset.maxWords)
        const queries    = generateQueries(10000, records, 0, preset.queryMaxLen)
        const suggest    = new LucidSuggest()
        suggest.setRecords(records)
        let offset = 0
        suite.add({
            name: `${name}, ${sampleSize} records`,
            defer: true,
            async fn(deferred) {
                await suggest.search(queries[offset])
                offset = (offset + 1) % queries.length
                deferred.resolve()
            },
        })
    }
}

suite.on('complete', function () {
    const results = {}
    this.forEach(({name: benchName, stats: {mean}}) => {
        const [nWords, nRecords]  = benchName.split(', ')
        results[nRecords]         = results[nRecords] || {}
        results[nRecords][nWords] = first2digits(mean * 1e6) + ' μs'
    })
    console.table(results)
})

suite.run()
