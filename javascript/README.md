# LucidSuggest

Embeddable search and autocomplete that works out of the box. Fast, simple, runs in browsers and NodeJS. Built with Rust and WebAssembly.

## Getting started

Install:
```shell
npm install lucid-suggest
```

Import and initialize:
```javascript
import LucidSuggest from 'lucid-suggest'

const suggest = new LucidSuggest()
suggest.setLanguage('en')
suggest.setRecords([
    {id: 1, title: "Electric Toothbrush"},
    {id: 2, title: "Lightning to USB C Cable"},
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
 {id: 2, title: "Lightning to USB C [C]able"},
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

Stemming is used to handle different word endings:
```javascript
await suggest.search("battery")
// returns:
[
 {id: 3, title: "AA Alkaline [Batteri]es"},
]
```

Particles receive special treatment, so they don't pop up every time you start typing a word:
```javascript
await suggest.search("to")
// returns:
[
    {id: 1, title: "Electric [To]othbrush"},
    {id: 2, title: "Lightning [to] USB C Cable"},
]
```


## Supported languages

- German (de)
- English (en)
- Spanish (es)
- Portuguese (pt)
- Russian (ru)


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

Optional `rating` field can be used to define an order of records that match a query equally well: records with greater rating will go higher in the result. Put data like priority, product popularity, or term frequency there to improve overall scoring.

For example, let's use state population as a rating:
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

### Bundle size

185K plain, 70K gzipped.

### Performance

If you target 60 fps in a browser, a search through 100 sentences will take 2-5% of a single frame or less, so you can expect negligible cost, even when calling it at every keystroke, without throttling/debouncing.

Full-fledged indexing is not implemented yet, so searching a larger number of records containing long texts can take too long at the moment.

Below are the detailed search performance measurements for record sets with different dimensions. Measured using Node.js 13.8, Intel Core i7 (I7-9750H) 2.6 GHz.

|              | 1-2 words | 4-8 words |
| -----------: | --------: | --------: |
|   10 records |     15 μs |     45 μs |
|  100 records |    140 μs |    440 μs |
| 1000 records |   1300 μs |   4300 μs |
