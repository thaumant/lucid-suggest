# LucidSuggest

Embeddable search and autocomplete that works out of the box. Fast, simple, runs in browsers and NodeJS. Built with Rust and WebAssembly.

## Getting started

Install:
```shell
npm install lucid-suggest
```

Import:
```javascript
import loadWasm from 'lucid-suggest'
```

Initialize:
```javascript
const {LucidSuggest} = await loadWasm()
const suggest = new LucidSuggest()
suggest.setRecords([
    {id: 1, title: "Nevada"},
    {id: 2, title: "New Jersey"},
    {id: 3, title: "New York"},
])
```

Search:
```javascript
suggest.search("nevada")
// returns:
[
 {id: 1, title: "[Nevada]"},
]
```

## Fulltext search features

Partial matches:
```javascript
suggest.search("new vegas")
// returns:
[
 {id: 2, title: "[New] Jersey"},
 {id: 3, title: "[New] York"},
]
```

Search as you type:
```javascript
suggest.search("new j")
// returns:
[
 {id: 2, title: "[New] [J]ersey"},
 {id: 3, title: "[New] York"},
]
```

Typo resilience:
```javascript
suggest.search("new jersy")
// returns:
[
 {id: 2, title: "[New] [Jersey]"},
 {id: 3, title: "[New] York"},
]
```

## Highlighting

By default LucidSuggest highlights matched parts of text using `[ ]`. You can pick your own markup:
```javascript
suggest.highlightWith('<strong>', '</strong>')
```

```javascript
suggest.search("new j")
// returns:
[
 {id: 2, title: "<strong>New</strong> <strong>J</strong>ersey"},
 {id: 3, title: "<strong>New</strong> York"},
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
suggest.search("ne")
// returns:
[
 {id: 3, title: "[Ne]w York"},
 {id: 2, title: "[Ne]w Jersey"},
 {id: 1, title: "[Ne]vada"},
]
```

### Performance

If you target 60 fps in a browser, a search through 100 sentences will take 2-5% of a single frame or less, so you can expect negligible cost, even when calling it at every keystroke, without throttling/debouncing.

Full-fledged indexing is not implemented yet, so searching a larger number of records containing long texts can take too long at the moment.

Below are the detailed search performance measurements for record sets with different dimensions. Measured using Node.js 13.8, Intel Core i7 (I7-9750H) 2.6 GHz.

|              | 1-2 words | 4-8 words |
| -----------: | --------: | --------: |
|   10 records |     14 μs |     40 μs |
|  100 records |    130 μs |    410 μs |
| 1000 records |   1100 μs |   4000 μs |
