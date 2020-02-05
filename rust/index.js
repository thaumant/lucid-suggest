const wasm = require('./pkg/lucid_suggest')

let RECORDS = []


function highlightUsing(left, right) {
    wasm.highlight_using(left, right)
}


function storeRecords(records) {
    RECORDS = records
    wasm.set_records(
        records.map(r => r.id),
        records.map(r => r.text).join('\0'),
    )
}

function search(query) {
    const ids = wasm.run_search(query)
    const highlights = wasm.get_highlights().split('\0')
    const hits = []
    for (let i = 0; i < ids.length; i++) {
        const id = ids[i]
        const highlighted = highlights[i]
        const record = RECORDS.find(r => r.id === id)
        if (!record)      throw new Error(`Missing record ${id}`)
        if (!highlighted) throw new Error(`Missing highlight for ${id}`)
        hits.push({...record, highlighted})
    }
    return hits
}


highlightUsing('[', ']')


module.exports = {
    highlightUsing,
    storeRecords,
    search,
}

// highlightUsing('<strong>', '</strong>')
storeRecords([
    {id: 1, text: 'Alaska'},
    {id: 2, text: 'Hawaii'},
    {id: 3, text: 'California'},
    {id: 4, text: 'Nevada'},
    {id: 5, text: 'Oregon'},
    {id: 6, text: 'Washington'},
    {id: 7, text: 'Arizona'},
    {id: 8, text: 'Colorado'},
    {id: 9, text: 'Idaho'},
    {id: 10, text: 'Montana'},
])
const hits = search('a')
console.log(hits)
