import {LangGerman} from "../../lang/lang-german"
import {tokenizeQuery} from "../index"


describe("tokenizeQuery", () => {
    test("Basic snapshot", () => {
        const lang      = new LangGerman()
        const source    = "Zahnbürste, mit 3D White, Weiß"
        const tokenized = tokenizeQuery(source, lang)
        expect(tokenized).toMatchSnapshot()
    })
})
