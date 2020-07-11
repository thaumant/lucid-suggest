import LangPlaceholder from "../../lang/LangPlaceholder"
import {WHITESPACE, PUNCTUATION} from "../../lang/cls"
import Word from "../Word"


describe("Word", () => {
    const lang = new LangPlaceholder()

    describe("split", () => {
        test("Empty", () => {
            const chars = ""
            const word  = new Word(chars.length)
            const split = word.split(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(split).toEqual([])
        })

        test("Single word, nothing to split", () => {
            const chars = "Foo"
            const word  = new Word(chars.length)
            const split = word.split(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(split).toMatchSnapshot()
        })

        test("Few words, some spaces and punctuation", () => {
            const chars = " Foo Bar, Baz; "
            const word  = new Word(chars.length)
            const split = word.split(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(split).toMatchSnapshot()
        })

        test("Unfinished flag, preserved", () => {
            const chars = "Foo Bar"
            const word  = new Word(chars.length)
            word.fin    = false
            const split = word.split(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(split).toHaveLength(2)
            expect(split[0].fin).toEqual(true)
            expect(split[1].fin).toEqual(false)
        })

        test("Unfinished flag, removed", () => {
            const chars = "Foo Bar "
            const word  = new Word(chars.length)
            word.fin    = false
            const split = word.split(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(split).toHaveLength(2)
            expect(split[0].fin).toEqual(true)
            expect(split[1].fin).toEqual(true)
        })
    })


    describe("Strip", () => {
        test("Empty", () => {
            const chars = ""
            const word  = new Word(chars.length)
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.slice).toEqual([0, 0])
        })

        test("Nothing to strip", () => {
            const chars = "Foo"
            const word  = new Word(chars.length)
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.slice).toEqual([0, 3])
        })

        test("Something to strip", () => {
            const chars = " Foo; "
            const word  = new Word(chars.length)
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.slice).toEqual([1, 4])
        })

        test("Word mid sentence", () => {
            const chars = " Foo, Bar, Baz "
            const word  = new Word(chars.length)
            word.slice  = [4, 11] // ", Bar, "
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.slice).toEqual([6, 9])
        })

        test("Strip everything", () => {
            const chars = " Foo, .?!, Baz "
            const word  = new Word(chars.length)
            word.slice  = [4, 11] // ", .?!, "
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.slice).toEqual([4, 4])
        })

        test("Unfinished flag removed", () => {
            const chars = " Foo, Bar, Baz "
            const word  = new Word(chars.length)
            word.slice  = [4, 11] // ", Bar, "
            word.fin    = false
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.fin).toEqual(true)
        })

        test("Unfinished flag preserved", () => {
            const chars = " Foo, Bar, Baz "
            const word  = new Word(chars.length)
            word.slice  = [4, 9] // ", Bar"
            word.fin    = false
            word.strip(chars, [WHITESPACE, PUNCTUATION], lang)
            expect(word.fin).toEqual(false)
        })
    })
})
