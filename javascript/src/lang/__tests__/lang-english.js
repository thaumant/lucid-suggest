import {LANG_ENGLISH as LANG} from "../lang-english"
import {CONSONANT, VOWEL} from '../cls'
import {ARTICLE} from "../pos"


describe("Lang English", () => {
    test("stem", () => {
        expect(LANG.stem("universe")).toEqual(7)
    })

    test("getPos", () => {
        expect(LANG.getPos("universe")).toEqual(undefined)
        expect(LANG.getPos("the")).toEqual(ARTICLE)
    })

    test("unicodeCompose", () => {
        expect(LANG.unicodeCompose("universe")).toEqual(undefined)
    })

    test("unicodeReduce", () => {
        expect(LANG.unicodeReduce("universe")).toEqual(undefined)
    })

    test("charClass", () => {
        expect(LANG.getCharClass('a')).toEqual(VOWEL)
        expect(LANG.getCharClass('n')).toEqual(CONSONANT)
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
            expect(normal).toHaveLength(1)
            expect(reduced).toHaveLength(1)
        }
    })
})