import {Lang} from './lang/lang-placeholder'
import {tokenizeRecord, tokenizeQuery} from './tokenization'
import {Word} from './tokenization/word'
import compileWasm, {WasmAPI} from './wasm-placeholder'

const DEFAULT_LIMIT = 10


export type Record = {
    id:      number,
    title:   string,
    rating?: number
}


export class LucidSuggest {
    id:         number
    lang:       Lang
    limit:      number
    records:    Map<number, Record>
    setupQueue: Promise<WasmAPI>

    constructor() {
        this.id         = 0
        this.lang       = new Lang()
        this.limit      = DEFAULT_LIMIT
        this.records    = new Map()
        this.setupQueue = compileWasm

        this.setup(wasm => {
            this.id = wasm.create_store()
        })
    }

    setup(fn: (api: WasmAPI) => unknown): Promise<void> {
        this.setupQueue = this.setupQueue.then(async wasm => {
            await fn(wasm)
            this.setupQueue = Promise.resolve(wasm)
            return wasm
        })
        return this.setupQueue.then(() => {})
    }

    destroy(): Promise<void> {
        const oldQueue = this.setupQueue
        this.setupQueue = Promise.reject(new Error('Suggest destroyed'))
        return oldQueue.then(wasm => {
            wasm.destroy_store(this.id)
        })
    }

    setLimit(limit: number): Promise<void> {
        return this.setup(wasm => {
            this.limit = limit
            wasm.set_limit(this.id, this.limit)
        })
    }

    addRecords(records: Record[]): Promise<void> {
        return this.setup(wasm => {
            for (const record of records) {
                const text  = tokenizeRecord(record.title, this.lang)
                wasm.add_record(
                    this.id,
                    record.id,
                    record.rating || 0,
                    text.source,
                    text.chars,
                    Uint32Array.from(text.classes),
                    serializeWords(text.words),
                )
                this.records.set(record.id, record)
            }
        })
    }

    async search(query: string): Promise<Hit[]> {
        const wasm  = await this.setupQueue
        const qtext = tokenizeQuery(query, this.lang)
        const len   = wasm.run_search(
            this.id,
            qtext.source,
            qtext.chars,
            Uint32Array.from(qtext.classes),
            serializeWords(qtext.words),
        )
        if (!len) return []
        const ids    = wasm.get_result_ids(this.id)
        const titles = wasm.get_result_titles(this.id).split('\0')
        const hits   = []
        if (len !== ids.length)    throw new Error(`Expected ${len} ids, got ${ids.length}`)
        if (len !== titles.length) throw new Error(`Expected ${len} titles, got ${titles.length}`)
        for (let i = 0; i < len; i++) {
            const id     = ids[i]
            const title  = titles[i]
            const record = this.records.get(id)
            if (!record) throw new Error(`Missing record ${id}`)
            hits.push(new Hit(title, record))
        }
        return hits
    }
}


export type HighlightedTextChunk = {
    text:      string,
    highlight: boolean,
}


export class Hit {
    record: Record
    chunks: HighlightedTextChunk[]

    constructor(title: string, record: Record) {
        this.record = record
        this.chunks = toChunks(title)
    }

    get title(): string {
        return highlight(this, '[', ']')
    }
}


export function highlight(hit: Hit, left: string, right: string) {
    let result = ''
    for (const {text, highlight} of hit.chunks) {
        result += highlight
            ? left + text + right
            : text
    }
    return result
}


function toChunks(title: string): HighlightedTextChunk[] {
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


function serializeWords(words: Word[]): Uint32Array {
    const serialized = new Uint32Array(words.length * 6)
    for (let i = 0; i < words.length; i++) {
        const word = words[i]
        serialized[i * 6 + 0] = word.offset
        serialized[i * 6 + 1] = word.slice[0]
        serialized[i * 6 + 2] = word.slice[1]
        serialized[i * 6 + 3] = word.stem
        serialized[i * 6 + 4] = word.pos
        serialized[i * 6 + 5] = +word.fin
    }
    return serialized
}