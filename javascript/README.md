# LucidSuggest

Embeddable search and autocomplete that works out of the box. Fast, simple, runs in browsers and NodeJS. Built with Rust and WebAssembly.

Note: this package is pre-1.0, it hasn't been battle-tested in production, and API may change.

## Live demo

[Check it out.](http://lucid-search.io.s3-website.eu-central-1.amazonaws.com/demo/index.html)


## Table of contents
- [Getting started](#getting-started)
- [Code examples](#code-examples)
- [Fulltext search features](#fulltext-search-features)
- [Rating](#rating)
- [Rendering results](#rendering-results)
- [Supported languages](#supported-languages)
- [Bundle sizes](#bundle-sizes)
- [Performance](#performance)


## Getting started

Install:
```shell
npm install lucid-suggest
```

Initialize:
```javascript
import {LucidSuggest} from 'lucid-suggest/en'

const suggest = new LucidSuggest()
suggest.setRecords([
    {id: 1, title: "Electric Toothbrush"},
    {id: 2, title: "Lightning to USB-C Cable"},
    {id: 3, title: "AA Alkaline Batteries"},
])
```

Search:
```javascript
await suggest.search("batteries")
// returns:
[
 {id: 3, title: "AA Alkaline [Batteries]"},
]
```

## Code examples

- [Browser and plain JavaScript](https://github.com/thaumant/lucid-suggest/tree/master/examples/browser-plain)
- [NodeJS and TypeScript](https://github.com/thaumant/lucid-suggest/tree/master/examples/node-ts)

## Fulltext search features

When an exact match is unavailable, the best possible partial matches are returned:
```javascript
await suggest.search("plastic toothbrush")
// returns:
[
 {id: 1, title: "Electric [Toothbrush]"},
]
```

Search as you type, results are provided from the first letter:
```javascript
await suggest.search("c")
// returns:
[
 {id: 2, title: "Lightning to USB-C [C]able"},
]
```

Typo resilience:
```javascript
await suggest.search("alcline bateries")
// returns:
[
 {id: 3, title: "AA [Alkaline] [Batteries]"},
]
```

```javascript
await suggest.search("tooth brush")
// returns:
[
 {id: 1, title: "Electric [Toothbrush]"},
]
```

Stemming is used to handle different word endings:
```javascript
await suggest.search("battery")
// returns:
[
 {id: 3, title: "AA Alkaline [Batteri]es"},
]
```

Function words (articles, prepositions, etc.) receive special treatment, so they don't pop up every time you start typing a word:
```javascript
await suggest.search("to")
// returns:
[
    {id: 1, title: "Electric [To]othbrush"},
    {id: 2, title: "Lightning [to] USB-C Cable"},
]
```

## Rating

Optional `rating` field can be used as a tie breaker: records with greater `rating` are ranked higher. Use priority, product popularity, or term frequency as `rating` to improve overall scoring.

For example, let's use state population as `rating`:
```javascript
suggest.setRecords([
    {id: 1, rating:  3000, title: "Nevada"},
    {id: 2, rating:  8900, title: "New Jersey"},
    {id: 3, rating: 19500, title: "New York"},
])
```

```javascript
await suggest.search("ne")
// returns:
[
 {id: 3, title: "[Ne]w York"},
 {id: 2, title: "[Ne]w Jersey"},
 {id: 1, title: "[Ne]vada"},
]
```


## Rendering results

By default LucidSuggest highlights matched parts of text using `[ ]`.

You can change this by providing an optional render function:

```javascript
const suggest = new LucidSuggest(chunks => {
    return chunks
        .map(({text, highlight}) => {
            return highlight ? `<strong>${text}</strong>` : text
        })
        .join('')
})

await suggest.search("battery")

// returns:
[{id: 3, title: "AA Alkaline <strong>Batteri</strong>es"}]
```

A helper function `highglightWith` is aimed to simplify render to a string:

```javascript
import {LucidSuggest, highlightWith} from 'lucid-suggest'

const suggest = new LucidSuggest(highlightWith('<strong>', '</strong>'))

await suggest.search("battery")

// returns:
[{id: 3, title: "AA Alkaline <strong>Batteri</strong>es"}]
```

For examples of rendering in React or Vue, see [Code examples](#code-examples) section.


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

At the moment LucidSuggest works best with shorter sentences, like shopping items or book titles. Using longer texts, like articles or movie descriptions, may lead to poor experience.

For example, for 10000 records, each containing 4-8 common English words, you can expect a typical search to take less than 1 ms, so you can simply call it at every keystroke, without throttling or Web Workers.

Below are the detailed performance measurements, obtained using Node.js 14.3, Intel Core i7 (I7-9750H) 2.6 GHz.

|                | 2-4 words | 4-8 words |
| -------------: | --------: | --------: |
|    100 records |   0.08 ms |   0.24 ms |
|   1000 records |   0.27 ms |   0.48 ms |
|  10000 records |   0.51 ms |   0.74 ms |
| 100000 records |   2.00 ms |   2.80 ms |
