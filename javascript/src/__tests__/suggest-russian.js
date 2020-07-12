import {LucidSuggest} from "../index"
import {LANG_RUSSIAN} from "../lang/lang-russian"


describe("Russian language", () => {
    const records = [
        {id: 10, title: "Батарейки алкалиновые — 12 штук в упаковке"},
        {id: 20, title: "Кабель Lightning — USB-A"},
        {id: 30, title: "Электрическая зубная щётка"},
        {id: 40, title: "Вакуумные компресионные мешки"},
    ]

    const suggest = new LucidSuggest(LANG_RUSSIAN)
    suggest.addRecords(records)

    test("Empty input", async () => {
        const hits = await suggest.search("")
        expect(hits).toMatchSnapshot()
    })

    test("Equality", async () => {
        const hits = await suggest.search("Электрическая зубная щетка")
        expect(hits).toMatchSnapshot()
    })

    test("Stemming", async () => {
        const hits = await suggest.search("зубной")
        expect(hits).toMatchSnapshot()
    })

    test("Partiles", async () => {
        const hits = await suggest.search("в")
        expect(hits).toMatchSnapshot()
    })

    test("Normalization", async () => {
        const hits = await suggest.search("ще")
        expect(hits).toMatchSnapshot()
    })
})