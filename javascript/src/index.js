import * as wasm from '../pkg/lucid_suggest_wasm_js'


export default class LucidSuggest {
    constructor() {
        this.id         = wasm.create_store()
        this.highlights = ['[', ']']
        this.records    = []
        this.highlightUsing('[', ']')
    }

    highlightUsing(left, right) {
        this.highlights[0] = left
        this.highlights[1] = right
        wasm.highlight_using(this.id, left, right)
        return this
    }

    addRecords(records) {
        records = setPrios(records)
        for (const record of records) {
            this.records.push(record)
        }
        wasm.add_records(
            this.id,
            records.map(r => r.id),
            records.map(r => r.text).join('\0'),
            records.map(r => r.prio),
        )
        return this
    }

    search(query) {
        wasm.run_search(this.id, query)
        const ids        = wasm.get_result_ids(this.id)
        const highlights = wasm.get_result_highlights(this.id).split('\0')
        const hits       = []
        for (let i = 0; i < ids.length; i++) {
            const id          = ids[i]
            const highlighted = highlights[i]
            const record      = this.records.find(r => r.id === id)
            if (!record)      throw new Error(`Missing record ${id}`)
            if (!highlighted) throw new Error(`Missing highlight for ${id}`)
            hits.push({...record, highlighted})
        }
        return hits
    }
}


function setPrios(records) {
    return records.some(r => r.prio != null && r.prio > 0)
        ? records.map(r => ({...r, prio: r.prio > 0 ? r.prio : 0}))
        : records.map((r, i) => ({...r, prio: records.length - i}))
}