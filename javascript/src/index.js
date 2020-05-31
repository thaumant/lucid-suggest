import compileWasm from '../pkg/lucid_suggest_wasm_js'

var NEXT_ID = 1

export default class LucidSuggest {
    constructor() {
        this.id         = NEXT_ID++
        this.dividers   = ['[', ']']
        this.records    = []
        this.setupQueue = compileWasm

        this.setup(wasm => {
            wasm.create_store(this.id)
        })
    }

    setup(fn) {
        this.setupQueue = this.setupQueue.then(async wasm => {
            await fn(wasm)
            this.setupQueue = Promise.resolve(wasm)
            return wasm
        })
        return this.setupQueue
    }

    destroy() {
        return this
            .setup(wasm => {
                wasm.destroy_store(this.id)
            })
            .then(() => {
                this.setupQueue = Promise.reject(new Error('Suggest destroyed'))
            })
    }

    setRecords(records) {
        return this.setup(wasm => {
            this.records = setRatings(records)
            wasm.set_records(
                this.id,
                this.records.map(r => r.id),
                this.records.map(r => r.title).join('\0'),
                this.records.map(r => r.rating),
            )
        })
    }

    highlightWith(left, right) {
        return this.setup(wasm => {
            this.dividers[0] = left
            this.dividers[1] = right
            wasm.highlight_with(this.id, left, right)
        })
    }

    async search(query) {
        const wasm = await this.setupQueue
        wasm.run_search(this.id, query)
        const ids    = wasm.get_result_ids(this.id)
        const titles = wasm.get_result_titles(this.id).split('\0')
        const hits   = []
        for (let i = 0; i < ids.length; i++) {
            const id     = ids[i]
            const title  = titles[i]
            const record = this.records.find(r => r.id === id)
            if (!record) throw new Error(`Missing record ${id}`)
            if (!title)  throw new Error(`Missing title for ${id}`)
            hits.push({...record, title})
        }
        return hits
    }
}


function setRatings(records) {
    return records.some(r => r.rating != null && r.rating > 0)
        ? records.map(r => ({...r, rating: r.rating > 0 ? r.rating : 0}))
        : records.map((r, i) => ({...r, rating: records.length - i}))
}
