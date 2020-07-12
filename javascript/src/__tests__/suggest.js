import {LucidSuggest, highlight} from "../index"
import {LANG_ENGLISH} from "../lang/lang-english"


describe("Suggest", () => {
    const records = [
        {id: 10, title: "Hello world!"},
        {id: 20, title: "Foo bar"},
        {id: 30, title: "-BAZZZ-"},
    ]

    const suggest = new LucidSuggest(LANG_ENGLISH)
    suggest.addRecords(records)

    test("Setup calls are sequential", async () => {
        function timeout(ms) {
            return new Promise((resolve) => {
                setTimeout(() => resolve(), ms)
            })
        }
        const steps = []
        suggest.setup(async () => {
            steps.push("start 1")
            await timeout(50)
            steps.push("end 1")
        })
        suggest.setup(async () => {
            steps.push("start 2")
            await timeout(50)
            steps.push("end 2")
        })
        await suggest.setupQueue()
        expect(steps).toEqual(["start 1", "end 1", "start 2", "end 2"])
    })

    test("Cannot search after destruction", async () => {
        const suggest = new LucidSuggest(LANG_ENGLISH)
        await suggest.addRecords(records)
        await suggest.search("foo")
        await suggest.destroy()
        await expect(suggest.search("foo")).rejects.toThrow("Suggest destroyed")
    })

    test("Empty input", async () => {
        const hits = await suggest.search("")
        expect(hits).toMatchSnapshot()
    })

    test("Equality", async () => {
        const hits = await suggest.search("foo bar")
        expect(hits).toMatchSnapshot()
    })

    test("Prefix", async () => {
        const hits = await suggest.search("ba")
        expect(hits).toMatchSnapshot()
    })

    test("Typos", async () => {
        const hits = await suggest.search("helo")
        expect(hits).toMatchSnapshot()
    })

    test("Typos", async () => {
        const hits = await suggest.search("helo")
        expect(hits).toMatchSnapshot()
    })

    test("Joined words", async () => {
        const hits = await suggest.search("foobar")
        expect(hits).toMatchSnapshot()
    })

    test("Limit", async () => {
        const hits1 = await suggest.search("b")
        expect(hits1).toHaveLength(2)

        await suggest.setLimit(1)
        const hits2 = await suggest.search("b")
        expect(hits2).toHaveLength(1)

        await suggest.setLimit(100)
        const hits3 = await suggest.search("b")
        expect(hits3).toHaveLength(2)
    })

    test("Rating", async () => {
        const suggest1 = new LucidSuggest(LANG_ENGLISH)
        suggest1.addRecords(records.map((r, i) => ({...r, rating: i})))
        const hits1 = await suggest1.search("")
        expect(hits1).toMatchSnapshot()

        const suggest2 = new LucidSuggest(LANG_ENGLISH)
        suggest2.addRecords(records.map((r, i) => ({...r, rating: records.length - i})))
        const hits2 = await suggest2.search("")
        expect(hits2).toMatchSnapshot()
    })

    describe("Render", () => {
        test("highlight helper", async () => {
            const suggest = new LucidSuggest(LANG_ENGLISH)
            suggest.addRecords(records)
            const hits = await suggest.search("hel")
            const rendered = hits.map(hit => highlight(hit, "((", "))"))
            expect(rendered).toMatchSnapshot()
        })

        test("DIY", async () => {
            const suggest = new LucidSuggest(LANG_ENGLISH)
            suggest.addRecords(records)
            const hits = await suggest.search("ba")
            const rendered = hits.map(hit => {
                return {
                    value: hit.record.id.toString(),
                    label: hit.chunks
                        .map(c => c.highlight ? "<<" + c.text + ">>" : c.text)
                        .join("")
                }
            })
            expect(rendered).toMatchSnapshot()
        })
    })
})