import {CharClass, ANY, NOTALPHA} from '../lang/cls'
import {UNKNOWN} from '../lang/pos'
import type Lang from '../lang/Lang'
import Word from './Word'


export default class Text {
    words:   Word[]
    source:  string
    chars:   string
    classes: CharClass[]


    constructor(source: string) {
        this.words    = [new Word(source.length)]
        this.source   = source
        this.chars    = source
        this.classes  = (new Array(source.length)).fill(ANY)
    }


    isEmpty() {
        return this.words.length === 0
    }


    setFin(fin: boolean): Text {
        if (this.words.length > 0) {
            this.words[this.words.length - 1].fin = fin
        }
        return this
    }


    normalize(lang: Lang): Text {
        if (this.isEmpty()) {
            return this
        }
        if (this.words.length > 1) {
            throw new Error("Normalization should always be the first step")
        }

        const nfc = lang.unicodeCompose(this.source)
        if (nfc != null) {
            this.source            = nfc
            this.chars             = nfc
            this.words[0].slice[1] = nfc.length
        }

        const reduced = lang.unicodeReduce(this.chars)
        if (reduced) {
            const [source, chars] = reduced
            this.source            = source
            this.chars             = chars
            this.words[0].slice[1] = chars.length
        }

        return this
    }


    split(pattern: CharClass[], lang: Lang): Text {
        const words = []
        let offset = 0
        for (const word of this.words) {
            for (const split of word.split(this.chars, pattern, lang)) {
                if (split.isEmpty()) continue
                split.offset = offset++
                words.push(split)
            }
        }
        this.words = words
        return this
    }


    strip(pattern: CharClass[], lang: Lang): Text {
        for (const word of this.words) {
            word.strip(this.chars, pattern, lang)
        }
        this.words = this.words.filter(w => !w.isEmpty())
        let offset = 0
        for (const word of this.words) {
            word.offset = offset++
        }
        return this
    }


    setStem(lang: Lang): Text {
        for (const word of this.words) {
            const chars = this.chars.slice(word.slice[0], word.slice[1])
            const stem  = lang.stem(chars)
            word.stem   = stem
        }
        return this
    }


    setPos(lang: Lang): Text {
        for (const word of this.words) {
            const chars = this.chars.slice(word.slice[0], word.slice[1])
            word.pos = lang.getPos(chars) || UNKNOWN
        }
        return this
    }


    setCharClasses(lang: Lang): Text {
        const classes: CharClass[] = []
        for (let i = 0; i < this.chars.length; i++) {
            const cls = lang.getCharClass(this.chars.charAt(i)) || NOTALPHA
            classes.push(cls)
        }
        this.classes = classes
        return this
    }


    lower(): Text {
        const lower = this.chars.toLowerCase()
        if (lower.length !== this.chars.length) {
            throw new Error("Lower case got different length")
        }
        this.chars = lower
        return this
    }
}
