import {LucidSuggest} from 'lucid-suggest/en'
import DATA from './e_commerce.json'

document.addEventListener('DOMContentLoaded', () => {
    const suggest = new LucidSuggest()
    suggest.highlightWith('<strong>', '</strong>')
    suggest.setRecords(DATA)

    function handleInputChange(query) {
        suggest.search(query)
            .then(hits => {
                resultsElement.innerHTML = hits
                    .slice(0, 10)
                    .map(hit => `<li class="list-group-item">${hit.title}</li>`)
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
