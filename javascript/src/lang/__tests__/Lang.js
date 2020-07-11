import {CONSONANT, VOWEL} from '../cls'
import {PARTICLE} from '../pos'
import Lang from '../Lang'
import Stemmer from '../stemming/ext/PlaceholderStemmer'


function langMock() {
    const composeList = [
        ["ó", "ó"]
    ]
    const reduceList = [
        ["ó", "o"],
        ["õ", "oo"],
    ]
    const posList = [
        [PARTICLE, "fóo"],
    ]
    const charClasses = [
        [CONSONANT, 'x'],
        [VOWEL, 'y']
    ]
    return new Lang(
        Stemmer,
        charClasses,
        posList,
        composeList,
        reduceList,
    )
}


describe("Lang", () => {
    const lang = langMock()


    describe("unicodeCompose", () => {
        test("empty", () => {
            const input  = ""
            const output = lang.unicodeCompose(input)
            expect(output).toEqual(undefined)
        })

        test("NFC", () => {
            const input  = "foóbar"
            const output = lang.unicodeCompose(input)
            expect(output).toEqual(undefined)
        })

        test("NFD", () => {
            const input  = "foóbar"
            const output = lang.unicodeCompose(input)
            expect(output).toEqual("foóbar")
        })
    })


    describe("unicodeReduce", () => {
        test("Empty", () => {
            const input  = ""
            const output = lang.unicodeReduce(input)
            expect(output).toEqual(undefined)
        })

        test("Noop", () => {
            const input  = "foobar"
            const output = lang.unicodeReduce(input)
            expect(output).toEqual(undefined)
        })

        test("NFC", () => {
            const input  = "foóbar"
            const output = lang.unicodeReduce(input)
            expect(output).toEqual(["foóbar", "foobar"])
        })

        test("Fill 0", () => {
            const input  = "fõbar"
            const output = lang.unicodeReduce(input)
            expect(output).toEqual(["fõ\0bar", "foobar"])
        })
    })


    describe("getPos", () => {
        test("Unknown", () => {
            const input  = "bar"
            const output = lang.getPos(input)
            expect(output).toEqual(undefined)
        })

        test("NFC", () => {
            const input  = "fóo"
            const output = lang.getPos(input)
            expect(output).toEqual(PARTICLE)
        })

        test("Reduced", () => {
            const input  = "foo"
            const output = lang.getPos(input)
            expect(output).toEqual(PARTICLE)
        })
    })


    describe("getCharClass", () => {
        test("Known", () => {
            const x = lang.getCharClass('x')
            const y = lang.getCharClass('y')
            expect(x).toEqual(CONSONANT)
            expect(y).toEqual(VOWEL)
        })

        test("Unknown", () => {
            const z = lang.getCharClass('z')
            expect(z).toEqual(undefined)
        })
    })
})
