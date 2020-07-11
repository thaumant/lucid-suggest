import {Lang} from "../../lang/lang-de"
import {tokenizeQuery} from "../index"


describe("tokenizeQuery", () => {
    test("Basic snapshot", () => {
        const lang      = new Lang()
        const source    = "Zahnbürste, mit 3D White, Weiß"
        const tokenized = tokenizeQuery(source, lang)
        expect(tokenized).toMatchSnapshot()
    })
})
