import LangDe from "../../lang/LangDe"
import {tokenizeQuery} from "../index"


describe("tokenizeQuery", () => {
    test("Basic snapshot", () => {
        const lang      = new LangDe()
        const source    = "Zahnbürste, mit 3D White, Weiß"
        const tokenized = tokenizeQuery(source, lang)
        expect(tokenized).toMatchSnapshot()
    })
})
