import * as wasm from '../pkg/lucid_suggest'


let RECORDS = []


export function highlightUsing(left, right) {
    wasm.highlight_using(left, right)
}


export function storeRecords(records) {
    RECORDS = records
    wasm.set_records(
        records.map(r => r.id),
        records.map(r => r.text).join('\0'),
    )
}

export function search(query) {
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
