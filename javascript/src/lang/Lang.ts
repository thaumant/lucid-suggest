import {Stemmer} from './stemming/ext/PlaceholderStemmer'
import {PartOfSpeech} from './pos'
import {
    CharClass,
    ANY,
    CONTROL,
    WHITESPACE,
    PUNCTUATION,
    NOTALPHA,
    NOTALPHANUM,
    CONSONANT,
    VOWEL,
} from './cls'


export class Lang {
    stemmer:    Stemmer
    classMap:   {[char: string]: CharClass}
    posMap:     {[char: string]: PartOfSpeech}
    composeMap: {[char: string]: string}
    reduceMap:  {[char: string]: string}

    constructor(
        stemmer:     Stemmer,
        classList:   [CharClass, string][],
        posList:     [PartOfSpeech, string][],
        composeList: [string, string][],
        reduceList:  [string, string][],
    ) {
        this.stemmer    = stemmer
        this.classMap   = {}
        this.posMap     = {}
        this.composeMap = {}
        this.reduceMap  = {}
        for (const [nfd, nfc] of composeList) {
            this.composeMap[nfd] = nfc
        }
        for (const [full, reduced] of reduceList) {
            this.reduceMap[full] = reduced
        }
        for (const [charClass, char] of classList) {
            this.classMap[char] = charClass
        }
        for (const [pos, word] of posList) {
            this.posMap[word] = pos
            const reduced = this.unicodeReduce(word)
            if (reduced) this.posMap[reduced[1]] = pos
        }
    }

    stem(word: string): number {
        this.stemmer.setCurrent(word)
        this.stemmer.stem()
        return this.stemmer.getCurrent().length
    }

    getPos(word: string): PartOfSpeech | undefined {
        return this.posMap[word]
    }

    getCharClass(char: string): CharClass | undefined {
        return this.classMap[char]
    }

    unicodeCompose(word: string): string | undefined {
        let result  = ''
        let changed = false
        for (let i = 0; i < word.length; i++) {
            const slice = word.slice(i, i + 2)
            const norm  = this.composeMap[slice]
            if (norm) {
                result += norm
                changed = true
                i++
            } else {
                result += slice.charAt(0)
            }
        }
        return changed ? result : undefined
    }

    unicodeReduce(word: string): [string, string] | undefined {
        let wordNormal  = ''
        let wordReduced = ''
        let changed     = false
        for (let i = 0; i < word.length; i++) {
            const charNormal  = word.charAt(i)
            const charReduced = this.reduceMap[charNormal]
            if (charReduced) {
                wordNormal += charNormal + '\0'.repeat(charReduced.length - charNormal.length)
                wordReduced += charReduced
                changed = true
            } else {
                wordNormal += charNormal
                wordReduced += charNormal
            }
        }
        return changed ? [wordNormal, wordReduced] : undefined
    }

    // https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
    // https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt

    charMatches(char: string, classes: CharClass[]): boolean {
        const code = char.charCodeAt(0)
        for (const cls of classes) {
            if (cls === ANY)         { return true }
            if (cls === CONTROL)     { if (this.isControl(code))     return true; continue }
            if (cls === WHITESPACE)  { if (this.isWhitespace(code))  return true; continue }
            if (cls === PUNCTUATION) { if (this.isPunctuation(code)) return true; continue }
            if (cls === NOTALPHA)    { if (this.isNotAlpha(char))    return true; continue }
            if (cls === NOTALPHANUM) { if (this.isNotAlphaNum(char)) return true; continue }
            if (cls === CONSONANT)   { if (this.isConsonant(char))   return true; continue }
            if (cls === VOWEL)       { if (this.isVowel(char))       return true; continue }
        }
        return false
    }

    isControl(code: number): boolean {
        if (code >= 0x00 && code <= 0x08) return true
        if (code >= 0x0e && code <= 0x1f) return true
        return false
    }

    isWhitespace(code: number): boolean {
        if (code >= 0x09 && code <= 0x0d) return true     // <control-0009>..<control-000d>
        if (code >= 0x2000 && code <= 0x200A) return true // en quad..hair space
        if (code === 0x20)   return true                  // space
        if (code === 0x85)   return true                  // <control-0085>
        if (code === 0xA0)   return true                  // no-break space
        if (code === 0x1680) return true                  // ogham space mark
        if (code === 0x2028) return true                  // line separator
        if (code === 0x2029) return true                  // paragraph separator
        if (code === 0x202F) return true                  // narrow no-break space
        if (code === 0x205F) return true                  // medium mathematical space
        if (code === 0x3000) return true                  // ideographic space
        return false
    }

    isPunctuation(code: number): boolean {
        if (code === 0x26)   return true // &
        if (code === 0x28)   return true // (
        if (code === 0x29)   return true // )
        if (code === 0x2c)   return true // ,
        if (code === 0x3a)   return true // :
        if (code === 0x3b)   return true // ;
        if (code === 0x2e)   return true // .
        if (code === 0x21)   return true // !
        if (code === 0x3f)   return true // ?
        if (code === 0x2d)   return true // -
        if (code === 0x2011) return true // ‑
        if (code === 0x2012) return true // ‒
        if (code === 0x2013) return true // –
        if (code === 0x2014) return true // —
        if (code === 0x2026) return true // …
        if (code === 0x203c) return true // ‼
        if (code === 0x2047) return true // ⁇
        if (code === 0x2048) return true // ⁈
        if (code === 0x2049) return true // ⁉
        return false
    }

    isNum(code: number): boolean {
        if (code >= 0x30 && code <= 0x39)  return true
        return false
    }

    isAlpha(char: string): boolean {
        const cls = this.getCharClass(char)
        if (cls === CONSONANT) return true
        if (cls === VOWEL)     return true
        return false
    }

    isNotAlpha(char: string): boolean {
        return !this.isAlpha(char)
    }

    isNotAlphaNum(char: string): boolean {
        if (this.isNum(char.charCodeAt(0))) return false
        if (this.isAlpha(char)) return false
        return true
    }

    isConsonant(char: string): boolean {
        return this.getCharClass(char) === CONSONANT
    }

    isVowel(char: string): boolean {
        return this.getCharClass(char) === VOWEL
    }
}
