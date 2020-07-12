import type {CharClass} from '../lang/cls'
import {
    PartOfSpeech,
    UNKNOWN,
    ARTICLE,
    PREPOSITION,
    CONJUNCTION,
    PARTICLE,
} from '../lang/pos'
import type {Lang} from '../lang/lang'


export class Word {
    offset: number
    slice:  [number, number]
    stem:   number
    pos:    PartOfSpeech
    fin:    boolean

    constructor(len: number) {
        this.offset = 0
        this.slice  = [0, len]
        this.stem   = len
        this.pos    = UNKNOWN
        this.fin    = true
    }

    isEmpty(): boolean {
        return this.slice[0] == this.slice[1]
    }

    isFunction(): boolean {
        if (this.pos === ARTICLE)     return true
        if (this.pos === PREPOSITION) return true
        if (this.pos === CONJUNCTION) return true
        if (this.pos === PARTICLE)    return true
        return false
    }

    dist(that: Word): number {
        const [left1, right1] = this.slice
        const [left2, right2] = that.slice
        if (left1 >= right2) { return left1 - right2 }
        if (left2 >= right1) { return left2 - right1 }
        throw new Error("Malformed words")
    }

    split(chars: string, pattern: CharClass[], lang: Lang): Word[] {
        const words = []
        let wordOffset = this.offset
        let charOffset = this.slice[0]
        while (charOffset < this.slice[1]) {
            while (true) {
                if (charOffset >= this.slice[1]) break
                if (!lang.charMatches(chars.charAt(charOffset), pattern)) break
                charOffset++
            }
            let len = 0
            while (true) {
                if (charOffset + len >= this.slice[1]) break
                if (lang.charMatches(chars.charAt(charOffset + len), pattern)) break
                len++
            }
            if (!len) {
                break
            }
            const word = new Word(len)
            word.offset   = wordOffset
            word.slice[0] = charOffset
            word.slice[1] = charOffset + len
            word.fin      = this.fin || charOffset + len < chars.length
            words.push(word)
            wordOffset += 1
            charOffset += len
        }
        return words
    }

    strip(chars: string, pattern: CharClass[], lang: Lang): Word {
        let [left, right] = this.slice
        while (true) {
            if (right <= left) break
            if (!lang.charMatches(chars.charAt(right - 1), pattern)) break
            right--
        }
        while (true) {
            if (left >= right) break
            if (!lang.charMatches(chars.charAt(left), pattern)) break
            left++
        }
        this.fin      = this.fin || right < this.slice[1]
        this.slice[0] = left
        this.slice[1] = right
        return this
    }
}
