# LucidSuggest

Autocomplete engine that works out of the box. Fast, easy to use, runs in browsers and Node.js. Built with Rust and WebAssembly.

Note: this package is pre-1.0, it hasn't been battle-tested in production, API may change.


## Table of contents
- [Resources](#resources)
- [Getting started](#getting-started)
- [Fulltext search features](#fulltext-search-features)
- [Rating](#rating)
- [Rendering results](#rendering-results)
- [Supported languages](#supported-languages)
- [Bundle sizes](#bundle-sizes)
- [Performance](#performance)


## Resources

[Live demo.](http://lucid-search.io.s3-website.eu-central-1.amazonaws.com/demo/index.html)

[API reference.](https://github.com/thaumant/lucid-suggest/tree/master/javascript/REFERENCE.md)

Code examples:
- [Plain JavaScript in browser](https://github.com/thaumant/lucid-suggest/tree/master/examples/browser-plain)
- [NodeJS](https://github.com/thaumant/lucid-suggest/tree/master/examples/node-ts)
- [React](https://github.com/thaumant/lucid-suggest/tree/master/examples/browser-react-ts)
- [Vue](https://github.com/thaumant/lucid-suggest/tree/master/examples/browser-vue)


## Getting started

Install:
```shell
npm install lucid-suggest
```

Initialize:
```javascript
import {LucidSuggest} from 'lucid-suggest/en'

const suggest = new LucidSuggest()
suggest.addRecords([
    {id: 1, title: "Electric Toothbrush"},
    {id: 2, title: "Lightning to USB-C Cable"},
    {id: 3, title: "AA Alkaline Batteries"},
])
```

Search:
```javascript
await suggest.search("batteries")
// returns:
// [
//   Hit { title: "AA Alkaline [Batteries]" }
// ]
```


## Rendering results

By default LucidSuggest highlights hit titles using `[ ]`.
The easiest way to change it is to use `highlight` helper function:

```javascript
import {LucidSuggest, highlight} from 'lucid-suggest'

const suggest = new LucidSuggest()
const hits = await suggest.search("to")

hits.map(hit => ({
    value: hit.record.id.toString(),
    label: highlight(hit, '<strong>', '</strong>')
}))
// returns:
// [
//   {value: "1", label: "Electric <strong>To</strong>othbrush"},
//   {value: "2", label: "Lightning <strong>to</strong> USB-C Cable"},
// ]
```

Or you can directly operate on chunks of a highlighted text,
which can come in handy if you need a more complex render logic:

```javascript
const hits = await suggest.search("to")
hits.map(hit => ({
    value: hit.record.id.toString(),
    label: hit.chunks
        .map(c => c.highlight ? `<strong>${c.text}</strong>` : c.text)
        .join('')
}))
// returns:
// [
//   {value: "1", label: "Electric <strong>To</strong>othbrush"},
//   {value: "2", label: "Lightning <strong>to</strong> USB-C Cable"},
// ]
```

For examples of rendering in React or Vue, see [Resources](#resources) section.

## Fulltext search features

When an exact match is unavailable, the best possible partial matches are returned:
```javascript
await suggest.search("plastic toothbrush")
// returns:
// [
//   Hit { title: "Electric [Toothbrush]" }
// ]
```

Search as you type, results are provided from the first letter:
```javascript
await suggest.search("c")
// returns:
// [
//   Hit { title: "Lightning to USB-C [C]able" }
// ]
```

Search algorithm is resilient to different kinds of typos:
```javascript
await suggest.search("alkln")
// returns:
// [
//   Hit { title: "AA [Alkalin]e Batteries" }
// ]
```

```javascript
await suggest.search("tooth brush")
// returns:
// [
//   Hit { title: "Electric [Toothbrush]" }
// ]
```

Stemming is used to handle different word forms:
```javascript
await suggest.search("battery")
// returns:
// [
//   Hit { title: "AA Alkaline [Batteri]es" }
// ]
```

Function words (articles, prepositions, etc.) receive special treatment, so they don't occupy top positions every time you start typing a word:
```javascript
await suggest.search("to")
// returns:
// [
//   Hit { title: "Electric [To]othbrush" },
//   Hit { title: "Lightning [to] USB-C Cable" },
// ]
```

## Rating

Optional `rating` field can be used as a tie breaker: records with greater `rating` are ranked higher. Use priority, product popularity, or term frequency as `rating` to improve overall scoring.

For example, let's use state population as `rating`:
```javascript
suggest.addRecords([
    {id: 1, rating:  3000, title: "Nevada"},
    {id: 2, rating:  8900, title: "New Jersey"},
    {id: 3, rating: 19500, title: "New York"},
])
```

```javascript
await suggest.search("ne")
// returns:
// [
//   Hit { title: "[Ne]w York" },
//   Hit { title: "[Ne]w Jersey" },
//   Hit { title: "[Ne]vada" },
// ]
```


## Supported languages

| Language   | Module             |
| :--------- | :----------------- |
| German     | `lucid-suggest/de` |
| English    | `lucid-suggest/en` |
| French     | `lucid-suggest/fr` |
| Spanish    | `lucid-suggest/es` |
| Portuguese | `lucid-suggest/pt` |
| Russian    | `lucid-suggest/ru` |


## Bundle sizes

| lang | size | gzipped |
| :--- | ---: | ------: |
| de   | 200K |     79K |
| en   | 202K |     80K |
| es   | 205K |     80K |
| es   | 208K |     82K |
| pt   | 205K |     81K |
| ru   | 202K |     79K |


## Performance

LucidSuggest works best with shorter sentences, like shopping items or book titles. Using longer texts, like articles or movie descriptions, may lead to performance regressions and generally poor user experience.

Below are the detailed performance measurements, obtained using Node.js 14.3, Intel Core i7 (I7-9750H) 2.6 GHz.

Searching:

|                 | 2-4 words | 4-8 words |
| --------------: | --------: | --------: |
|     100 records |   0.09 ms |   0.24 ms |
|    1000 records |   0.27 ms |   0.48 ms |
|  10 000 records |   0.45 ms |   0.68 ms |
| 100 000 records |   1.40 ms |   2.00 ms |

Indexing:

|                 | 2-4 words | 4-8 words |
| --------------: | --------: | --------: |
|     100 records |      1 ms |      2 ms |
|    1000 records |     11 ms |     16 ms |
|  10 000 records |     88 ms |    160 ms |
| 100 000 records |    810 ms |   1900 ms |
