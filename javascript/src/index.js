import compileWasm from '../pkg/lucid_suggest_wasm'

const DEFAULT_LIMIT = 10
var NEXT_ID = 1


export class LucidSuggest {
    constructor() {
        this.id         = NEXT_ID++
        this.limit      = DEFAULT_LIMIT
        this.records    = new Map()
        this.setupQueue = compileWasm

        this.setup(wasm => {
            wasm.create_store(this.id)
            wasm.highlight_with(this.id, '{{', '}}')
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
            records = setRatings(records)
            wasm.set_records(
                this.id,
                records.map(r => r.id),
                records.map(r => r.title).join('\0'),
                records.map(r => r.rating),
            )
            for (const record of records) {
                this.records.set(record.id, record)
            }
        })
    }

    setLimit(limit) {
        return this.setup(wasm => {
            this.limit = limit
            wasm.set_limit(this.id, this.limit)
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
            const record = this.records.get(id)
            if (!record) throw new Error(`Missing record ${id}`)
            if (!title)  throw new Error(`Missing title for ${id}`)
            hits.push(new Hit(title, record))
        }
        return hits
    }
}


export function highlight(hit, left, right) {
    let result = ''
    for (const {text, highlight} of hit.chunks) {
        result += highlight
            ? left + text + right
            : text
    }
    return result
}


export class Hit {
    constructor(title, record) {
        this.record = record
        this.chunks = toChunks(title)
    }

    get title() {
        return highlight(this, '[', ']')
    }
}


function toChunks(title) {
    const split  = title.split(/{{|}}/g)
    const chunks = []
    for (let i = 0; i < split.length; i++) {
        if (split[i] != '') {
            chunks.push({
                text: split[i],
                highlight: i % 2 === 1,
            })
        }
    }
    return chunks
}


function setRatings(records) {
    return records.some(r => r.rating != null && r.rating > 0)
        ? records.map(r => ({...r, rating: r.rating > 0 ? r.rating : 0}))
        : records.map((r, i) => ({...r, rating: records.length - i}))
}
