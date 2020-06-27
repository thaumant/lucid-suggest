# LucidSuggest

Embeddable search and autocomplete that works out of the box. Fast, simple, runs in browsers and NodeJS. Built with Rust and WebAssembly.

This package hasn't been battle-tested in production. For data restrictions see [Performance section](#performance) below.

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
await suggest.search("alcaline bateries")
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


## Highlighting

By default LucidSuggest highlights matched parts of text using `[ ]`. You can pick your own markup:
```javascript
suggest.highlightWith('<strong>', '</strong>')
```

```javascript
await suggest.search("battery")
// returns:
[
 {id: 3, title: "AA Alkaline <strong>Batteri</strong>es"},
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


## Supported languages

| Language   | Module             |
| :--------- | :----------------- |
| German     | `lucid-suggest/de` |
| English    | `lucid-suggest/en` |
| Spanish    | `lucid-suggest/es` |
| Portuguese | `lucid-suggest/pt` |
| Russian    | `lucid-suggest/ru` |


## TypeScript

When using TypeScript you have to import the empty root module, otherwise TypeScript compiler will not read `typings.d.ts` containing submodule declarations.
```
import 'lucid-suggest'
import {LucidSuggest} from 'lucid-suggest/en'
```


## Bundle sizes

| lang | size | gzipped |
| :--- | ---: | ------: |
| de   | 148K |     60K |
| en   | 149K |     61K |
| es   | 153K |     62K |
| pt   | 153K |     62K |
| ru   | 149K |     61K |


## Performance

At the moment LucidSuggest works best with shorter sentences, like shopping items or book titles. Using longer texts, like articles or movie descriptions, may lead to poor experience.

Also, full-fledged indexing is not implemented yet, so searching a large number of records can also take too long.

For example, for 1000 records, each containing 4-8 common English words you can expect a search to take up to 7ms, as illustrated in the table below. That's less than a half of a frame if you target 60 FPS. You can call it at every keystroke, without using throttling or Web Workers.

Below are the detailed performance measurements, obtained using Node.js 14.3, Intel Core i7 (I7-9750H) 2.6 GHz.

|              | 1-2 words | 4-8 words |
| -----------: | --------: | --------: |
|   10 records |   0.02 ms |   0.07 ms |
|  100 records |   0.13 ms |   0.78 ms |
| 1000 records |   1.20 ms |   7.10 ms |
