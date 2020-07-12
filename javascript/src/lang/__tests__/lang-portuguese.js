import {LANG_PORTUGUESE as LANG} from "../lang-portuguese"
import {CONSONANT, VOWEL} from '../cls'
import {ARTICLE} from "../pos"


describe("Lang Portuguese", () => {
    test("stem", () => {
        expect(LANG.stem("quilométricas")).toEqual(9)
    })

    test("getPos", () => {
        expect(LANG.getPos("quilométricas")).toEqual(undefined)
        expect(LANG.getPos("uma")).toEqual(ARTICLE)
    })

    test("unicodeCompose", () => {
        expect(LANG.unicodeCompose("conforme")).toEqual(undefined)
        expect(LANG.unicodeCompose("conceição")).toEqual("conceição")
    })

    test("unicodeReduce", () => {
        expect(LANG.unicodeReduce("conforme")).toEqual(undefined)
        expect(LANG.unicodeReduce("Conceição")).toEqual(["Conceição", "Conceicao"])
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