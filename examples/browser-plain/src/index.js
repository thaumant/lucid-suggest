import {LucidSuggest} from 'lucid-suggest/en'
import DATA from './e_commerce.json'

document.addEventListener('DOMContentLoaded', () => {
    const suggest = new LucidSuggest()
    suggest.setRecords(DATA)

    function renderHit(hit) {
        const title = hit.chunks
            .map(chunk => {
                return chunk.highlight
                    ? `<strong>${chunk.text}</strong>`
                    : chunk.text
            })
            .join('')
        return `<li class="list-group-item">${title}</li>`
    }

    function handleInputChange(query) {
        suggest.search(query)
            .then(hits => {
                resultsElement.innerHTML = hits
                    .slice(0, 10)
                    .map(renderHit)
                    .join('\n')
            })
    }

    const inputElement   = document.querySelector('#search-input')
    const resultsElement = document.querySelector('#search-results')

    handleInputChange(inputElement.value)

    inputElement.addEventListener('input', event => {
        handleInputChange(event.target.value)
    })
})
