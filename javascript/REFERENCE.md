# LucidSuggest API reference


## Table of contents
- [LucidSuggest class](#lucidsuggest-class)
- [Record type](#record-type)
- [Hit type](#hit-type)
- [HighlightedTextChunk type](#highlightedtextchunk-type)
- [highlight function](#highlight-function)


## LucidSuggest class

Methods:

| Name       | Type                                 | Description                                       |
| :--------- | :----------------------------------- | :------------------------------------------------ |
| addRecords | `(records: Record[]): Promise<void>` | Add records to the search index.                  |
| setLimit   | `(limit: number): Promise<void>`     | Set the number of top hits returned.              |
| search     | `(query: string): Promise<Hit[]>`    | Get top hits (matched records) for a given query. |
| destroy    | `(): Promise<void>`                  | Destroy the instance and clean it's memory.       |

**Note:** `setLimit` and `addRecords` operate by pushing tasks into a setup queue.
You can skip awaiting them because `search` will wait for that queue to finish.
Although if you pass malformed arguments you will get an uncaught exception.

**Note:** don't throw away `LucidSuggest` instance without calling `destroy`.
WebAssembly doesn't have garbage collection, so the instance and
all the associated data will remain in memory, resulting in memory leak.

An example of usage:
```javascript
const suggest = new LucidSuggest()
suggest.setLimit(5)
suggest.addRecords([
    {id: 1, title: "Electric Toothbrush"},
    {id: 2, title: "Lightning to USB-C Cable"},
    {id: 3, title: "AA Alkaline Batteries"},
])
const hits = await suggest.search("electr")
// returns:
// [
//   Hit { title: "[Electr]ic Toothbrush" }
// ]
```


## Record type

An object stored in `LucidSuggest` instance and matched with a query by it's `search` method.

Properties:

| Name   | Type                 | Description                                                         |
| :----- | :------------------- | :------------------------------------------------------------------ |
| id     | `number`             | Unique non-negative integer identifier.                             |
| title  | `string`             | Text used for fulltext search.                                      |
| rating | `number | undefined` | Matching tie breaker: records with greater rating are ranked higher |

**Note:** use priority, product popularity, or term frequency as `rating` to improve overall scoring.


## Hit type

An object containing information on a record-query match, including the record itself.

Properties:

| Name   | Type                     | Description                                                            |
| :----- | :----------------------- | :--------------------------------------------------------------------- |
| title  | `string`                 | The record title highlighted with default `[ ]`. Useful for debugging. |
| chunks | `HighlightedTextChunk[]` | An object representation of the matched title.                         |
| record | `Record`                 | The original record linked.                                            |

An example of record structure:
```javascript
{
    title: '[Electr]ic toothbrush',
    chunks: [
        {text: 'Electr',        highlight: true},
        {text: 'ic toothbrush', highlight: false},
    ],
    record: {
        id: 1,
        title: "Electric Toothbrush",
    },
}
```


## HighlightedTextChunk type

An object representation of a piece of a matched text.

Properties:

| Name      | Type      | Description                            |
| :-------- | :-------- | :------------------------------------- |
| text      | `string`  | Substring of a record title.           |
| highlight | `bool`    | Should the substring be highlighted.   |

An example of chunk structure:
```javascript
{
    text: "Electr",
    highlight: true,
}
```


## highlight function

Type: `highlight(hit: Hit, left: string, right: string): string`.

A helper function for quick formatting.

An example of usage:
```javascript
import {LucidSuggest, highlight} from 'lucid-suggest'

const suggest = new LucidSuggest()
const hits = await suggest.search("to")

hits.map(hit => highlight(hit, '<strong>', '</strong>'))
// returns:
// [
//   "Electric <strong>To</strong>othbrush",
//   "Lightning <strong>to</strong> USB-C Cable",
// ]
```
