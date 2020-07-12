import {LANG_GERMAN} from "../../lang/lang-german"
import {tokenizeQuery} from "../index"


describe("tokenizeQuery", () => {
    test("Basic snapshot", () => {
        const source    = "Zahnbürste, mit 3D White, Weiß"
        const tokenized = tokenizeQuery(source, LANG_GERMAN)
        expect(tokenized).toMatchSnapshot()
    })
})
