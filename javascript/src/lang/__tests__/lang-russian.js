import {LANG_RUSSIAN as LANG} from "../lang-russian"
import {CONSONANT, VOWEL} from '../cls'
import {PARTICLE} from "../pos"


describe("Lang Russian", () => {
    test("stem", () => {
        expect(LANG.stem("важный")).toEqual(4)
    })

    test("getPos", () => {
        expect(LANG.getPos("важный")).toEqual(undefined)
        expect(LANG.getPos("ведь")).toEqual(PARTICLE)
    })

    test("unicodeCompose", () => {
        expect(LANG.unicodeCompose("важный")).toEqual(undefined)
        expect(LANG.unicodeCompose("Ёлка")).toEqual("Ёлка")
    })

    test("unicodeReduce", () => {
        expect(LANG.unicodeReduce("важный")).toEqual(undefined)
        expect(LANG.unicodeReduce("Ёлка")).toEqual(["Ёлка", "Елка"])
    })

    test("charClass", () => {
        expect(LANG.getCharClass('ы')).toEqual(VOWEL)
        expect(LANG.getCharClass('ф')).toEqual(CONSONANT)
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