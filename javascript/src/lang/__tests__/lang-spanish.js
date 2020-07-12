import {LANG_SPANISH as LANG} from "../lang-spanish"
import {CONSONANT, VOWEL} from '../cls'
import {ARTICLE} from "../pos"


describe("Lang Spanish", () => {
    test("stem", () => {
        expect(LANG.stem("torniquete")).toEqual(9)
    })

    test("getPos", () => {
        expect(LANG.getPos("torniquete")).toEqual(undefined)
        expect(LANG.getPos("una")).toEqual(ARTICLE)
    })

    test("unicodeCompose", () => {
        expect(LANG.unicodeCompose("torniquete")).toEqual(undefined)
        expect(LANG.unicodeCompose("piñata")).toEqual("piñata")
    })

    test("unicodeReduce", () => {
        expect(LANG.unicodeReduce("torniquete")).toEqual(undefined)
        expect(LANG.unicodeReduce("piñata")).toEqual(["piñata", "pinata"])
    })

    test("charClass", () => {
        expect(LANG.getCharClass('n')).toEqual(CONSONANT)
        expect(LANG.getCharClass('a')).toEqual(VOWEL)
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