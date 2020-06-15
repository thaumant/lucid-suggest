import compileWasm from '../pkg/lucid_suggest_wasm'

const DEFAULT_LIMIT = 10
var NEXT_ID = 1


export class LucidSuggest {
    constructor() {
        this.id         = NEXT_ID++
        this.limit      = DEFAULT_LIMIT
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
        const oldQueue = this.setupQueue
        this.setupQueue = Promise.reject(new Error('Suggest destroyed'))
        oldQueue.then(wasm => {
            wasm.destroy_store(this.id)
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

    setLimit(limit) {
        return this.setup(wasm => {
            this.limit = limit
            wasm.set_limit(this.id, this.limit)
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
