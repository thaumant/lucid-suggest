import type {Word} from './tokenization/word'
import type {HighlightedTextChunk} from './index'


export function once<T>(fn: () => T): () => T {
    let result: T | undefined = undefined
    return () => {
        if (result !== undefined) {
            return result
        } else {
            result = fn()
            return result
        }
    }
}


export function toChunks(title: string): HighlightedTextChunk[] {
    const split  = title.split(/{{|}}/g)
    const chunks = []
    for (let i = 0; i < split.length; i++) {
        if (split[i] != '') {
            chunks.push({
                text: split[i],
                highlight: i % 2 === 1,
            })
        }
    }
    return chunks
}


export function serializeWords(words: Word[]): Uint32Array {
    const serialized = new Uint32Array(words.length * 6)
    for (let i = 0; i < words.length; i++) {
        const word = words[i]
        serialized[i * 6 + 0] = word.offset
        serialized[i * 6 + 1] = word.slice[0]
        serialized[i * 6 + 2] = word.slice[1]
        serialized[i * 6 + 3] = word.stem
        serialized[i * 6 + 4] = word.pos
        serialized[i * 6 + 5] = +word.fin
    }
    return serialized
}