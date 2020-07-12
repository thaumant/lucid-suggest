import {LANG_GERMAN as LANG} from "../lang-german"
import {CONSONANT, VOWEL} from '../cls'
import {ARTICLE} from "../pos"


describe("Lang German", () => {
    test("stem", () => {
        expect(LANG.stem("singen")).toEqual(4)
    })

    test("getPos", () => {
        expect(LANG.getPos("singen")).toEqual(undefined)
        expect(LANG.getPos("das")).toEqual(ARTICLE)
    })

    test("unicodeCompose", () => {
        expect(LANG.unicodeCompose("singen")).toEqual(undefined)
        expect(LANG.unicodeCompose("mädchen")).toEqual("mädchen")
    })

    test("unicodeReduce", () => {
        expect(LANG.unicodeReduce("singen")).toEqual(undefined)
        expect(LANG.unicodeReduce("mädchen")).toEqual(["mädchen", "madchen"])
        expect(LANG.unicodeReduce("straße")).toEqual(["straß\0e", "strasse"])
    })

    test("charClass", () => {
        expect(LANG.getCharClass('a')).toEqual(VOWEL)
        expect(LANG.getCharClass('n')).toEqual(CONSONANT)
        expect(LANG.getCharClass('ß')).toEqual(CONSONANT)
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
            if (normal === 'ẞ') { continue; }
            if (normal === 'ß') { continue; }
            expect(normal).toHaveLength(1)
            expect(reduced).toHaveLength(1)
        }
    })
})