import {LANG_FRENCH as LANG} from "../lang-french"
import {CONSONANT, VOWEL} from '../cls'
import {ARTICLE} from "../pos"


describe("Lang French", () => {
    test("stem", () => {
        expect(LANG.stem("université")).toEqual(7)
    })

    test("getPos", () => {
        expect(LANG.getPos("universite")).toEqual(undefined)
        expect(LANG.getPos("les")).toEqual(ARTICLE)
    })

    test("unicodeCompose", () => {
        expect(LANG.unicodeCompose("université")).toEqual(undefined)
        expect(LANG.unicodeCompose("château")).toEqual("château")
    })

    test("unicodeReduce", () => {
        expect(LANG.unicodeReduce("univers")).toEqual(undefined)
        expect(LANG.unicodeReduce("château")).toEqual(["château", "chateau"])
        expect(LANG.unicodeReduce("sœur")).toEqual(["sœ\0ur", "soeur"])
    })

    test("charClass", () => {
        expect(LANG.getCharClass('n')).toEqual(CONSONANT)
        expect(LANG.getCharClass('a')).toEqual(VOWEL)
        expect(LANG.getCharClass('ô')).toEqual(VOWEL)
        expect(LANG.getCharClass('œ')).toEqual(VOWEL)
        expect(LANG.getCharClass('%')).toEqual(undefined)
    })

    test("composeMap dimensions", () => {
        for (const [nfd, nfc] of Object.entries(LANG.composeMap)) {
            expect(nfd).toHaveLength(2)
            expect(nfc).toHaveLength(1)
        }
    })

    test("reduceMap dimensions", () => {
        for (const [normal, reduced] of Object.entries(LANG.reduceMap)) {
            if (['Æ', 'æ', 'Œ', 'œ', 'Ø', 'ø'].includes(normal)) { continue; }
            expect(normal).toHaveLength(1)
            expect(reduced).toHaveLength(1)
        }
    })
})