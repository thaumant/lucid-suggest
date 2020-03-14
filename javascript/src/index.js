import compile from '../pkg/lucid_suggest_wasm_js'

export default compile.then(function(wasm) {
    function setRatings(records) {
        return records.some(r => r.rating != null && r.rating > 0)
            ? records.map(r => ({...r, rating: r.rating > 0 ? r.rating : 0}))
            : records.map((r, i) => ({...r, rating: records.length - i}))
    }

    return class LucidSuggest {
        constructor() {
            this.id       = wasm.create_store()
            this.dividers = ['[', ']']
            this.records  = []
            this.highlightWith('[', ']')
        }

        highlightWith(left, right) {
            this.dividers[0] = left
            this.dividers[1] = right
            wasm.highlight_with(this.id, left, right)
            return this
        }

        setRecords(records) {
            records = setRatings(records)
            for (const record of records) {
                this.records.push(record)
            }
            wasm.set_records(
                this.id,
                records.map(r => r.id),
                records.map(r => r.title).join('\0'),
                records.map(r => r.rating),
            )
            return this
        }

        search(query) {
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
})

