import {LucidSuggest} from "../index"
import {LANG_GERMAN} from "../lang/lang-german"


describe("German language", () => {
    const records = [
        {id: 10, title: "AA Batterien, Alkaline — 12er Pack"},
        {id: 20, title: "Lightning auf USB A Kabel"},
        {id: 30, title: "Zahnbürste, mit 3D White, Weiß"},
        {id: 40, title: "Hi-Fi-Mitteltöner, Speaker für die Mittelton-Wiedergabe"},
    ]

    const suggest = new LucidSuggest(LANG_GERMAN)
    suggest.addRecords(records)

    test("Empty input", async () => {
        const hits = await suggest.search("")
        expect(hits).toMatchSnapshot()
    })

    test("Equality", async () => {
        const hits = await suggest.search("lightning auf usb a kabel")
        expect(hits).toMatchSnapshot()
    })

    test("Stemming", async () => {
        const hits = await suggest.search("batterie aa")
        expect(hits).toMatchSnapshot()
    })

    test("Partiles", async () => {
        const hits = await suggest.search("mit")
        expect(hits).toMatchSnapshot()
    })

    test("Normalization", async () => {
        const hits1 = await suggest.search("mittelto")
        const hits2 = await suggest.search("mitteltö")
        const hits3 = await suggest.search("mitteltö")
        expect(hits1).toMatchSnapshot()
        expect(hits2).toMatchSnapshot()
        expect(hits3).toMatchSnapshot()
    })
})