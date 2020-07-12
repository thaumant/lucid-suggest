import {LucidSuggest, Hit} from 'lucid-suggest/en'
import DATA from './e_commerce.json'
import React, {useState, useEffect} from 'react'
import IconLoupe from './IconLoupe'
import ListItem from './ListItem'

const suggest: LucidSuggest = new LucidSuggest()
suggest.addRecords(DATA)

const App: React.FC = () => {
    const [input, setInput] = useState<string>('')
    const [hits, setHits] = useState<Hit[]>([])

    useEffect(() => {
        suggest.search(input).then(setHits)
    }, [input])

    return (
        <>
            <h1>LucidSuggest demo</h1>
            <br />
            <form>
                <div className="input-group mb-3">
                    <input
                        value={input}
                        className="form-control"
                        type="text"
                        onChange={e => setInput(e.target.value)}
                    />
                    <div className="input-group-append">
                        <span className="input-group-text" id="basic-addon2">
                            <IconLoupe />
                        </span>
                    </div>
                </div>
            </form>
            <ul className="list-group" id="search-results">
                {hits.map(hit => (
                    <ListItem key={hit.record.id} hit={hit} />
                ))}
            </ul>
        </>
    )
}

export default App
