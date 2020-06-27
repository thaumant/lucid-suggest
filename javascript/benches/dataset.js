const fs = require('fs')

const CHARS = 'abcdefghijklmnopqrstuvwxyz'.split('')

const WORDS = fs.readFileSync(__dirname + '/../../datasets/top_1000_words_en.csv', 'utf8')
    .split('\n')
    .map(word => word.trim())
    .filter(word => word.length > 0)

const DISTRIBUTION = WORDS
    .map((_, ix) => {
        const freq = Math.ceil((1000 - ix) / 100) // linear from 10x to 1x
        return [ix, freq]
    })
    .reduce((dist, [ix, freq]) => {
        for (let i = 0; i < freq; i++) {
            dist.push(ix)
        }
        return dist
    }, [])


function generateRecords(size, minWords, maxWords) {
    let records = []
    for (let i = 0; i < size; i++) {
        records.push({
            id: i + 1,
            title: generateTitle(minWords, maxWords),
        })
    }
    return records
}


function generateQueries(size, records, minLen = 1.0, maxLen = 1.0) {
    const queries = []
    for (let i = 0; i < size; i++) {
        const record = randItem(records)
        const query  = corrupt(record.title)
        const prefix = Math.ceil(randRangeFloat(query.length * minLen, query.length * maxLen))
        queries.push(query.slice(0, prefix))
    }
    return queries
}


function generateTitle(minWords, maxWords) {
    const len = randRangeInt(minWords, maxWords)
    let title = ''
    for (let j = 0; j < len; j++) {
        title += randWord()
        title += Math.random() > 0.2 ? ' ' : ', '
    }
    title = title[0].toUpperCase() + title.slice(1)
    title = title.slice(
        0,
        title[title.length - 2] === ',' ? -2 : -1,
    )
    return title
}


function corrupt(title) {
    const chars = title
    .toLowerCase()
    .replace(/, /, ' ')
    .split('')

    const typos = randRangeInt(0, Math.ceil(title.length / 10))

    for (let i = 0; i < typos; i++) {
        let ix = 0
        while (ix == 0 || chars[ix - 1] == ' ') {
            ix = randRangeInt(0, chars.length - 1)
        }
        const rand = Math.random()
        if      (rand >= 0.667) chars[ix] = randItem(CHARS)
        else if (rand >= 0.333) chars.splice(ix, 1)
        else if (rand >= 0.000) chars.splice(ix, 0, randItem(CHARS))
    }

    return chars.join('')
}


function randItem(arr) {
    return arr[Math.floor(Math.random() * arr.length)]
}


function randRangeInt(min, max) {
    return Math.floor(Math.random() * (max + 1 - min)) + min
}


function randRangeFloat(min, max) {
    return Math.random() * (max + 1 - min) + min
}


function randWord() {
    const ix   = randItem(DISTRIBUTION)
    const word = WORDS[ix]
    return word
}


exports.generateRecords = generateRecords
exports.generateQueries = generateQueries
