import {LucidSuggest} from 'lucid-suggest/en'
import DATA from './e_commerce.json'

const COUNT = DATA.length

function perfStart() {
    performance.clearMarks()
    performance.clearMeasures()
    performance.mark('search_start')
}

function perfEnd() {
    performance.mark('search_end')
    performance.measure('search', 'search_start', 'search_end')
    const measurements = performance.getEntriesByName('search')
    const measurement  = measurements[measurements.length - 1]
    return Math.round(measurement.duration * 10) / 10
}

document.addEventListener('DOMContentLoaded', () => {
    const suggest = new LucidSuggest()
    suggest.addRecords(DATA)

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
        perfStart()
        suggest.search(query)
            .then(hits => {
                statsElement.innerHTML = `${perfEnd()} ms over ${COUNT} records`
                resultsElement.innerHTML = hits
                    .slice(0, 10)
                    .map(renderHit)
                    .join('\n')
            })
    }

    const statsElement   = document.querySelector('#search-stats')
    const inputElement   = document.querySelector('#search-input')
    const resultsElement = document.querySelector('#search-results')

    handleInputChange(inputElement.value)

    inputElement.addEventListener('input', event => {
        handleInputChange(event.target.value)
    })
})
