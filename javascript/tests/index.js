const {LucidSuggest, highlight} = require('../en')

describe('Suggest', () => {
    const records = [
        {id: 10, title: 'Hello world!'},
        {id: 20, title: 'Foo bar'},
        {id: 30, title: '-BAZZZ-'},
    ]

    const suggest = new LucidSuggest()
    suggest.setRecords(records)

    test('Setup calls are sequential', async () => {
        function timeout(ms) {
            return new Promise((resolve) => {
                setTimeout(() => resolve(), ms)
            })
        }
        const steps = []
        suggest.setup(async () => {
            steps.push('start 1')
            await timeout(50)
            steps.push('end 1')
        })
        suggest.setup(async () => {
            steps.push('start 2')
            await timeout(50)
            steps.push('end 2')
        })
        await suggest.setupQueue
        expect(steps).toEqual(['start 1', 'end 1', 'start 2', 'end 2'])
    })

    test('Cannot search after destruction', async () => {
        const suggest = new LucidSuggest()
        await suggest.setRecords(records)
        await suggest.search('foo')
        await suggest.destroy()
        await expect(suggest.search('foo')).rejects.toThrow('Suggest destroyed')
    })

    test('Empty input', async () => {
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', async () => {
        const hits = await suggest.search('foo bar')
        expect(hits).toMatchSnapshot()
    })

    test('Prefix', async () => {
        const hits = await suggest.search('ba')
        expect(hits).toMatchSnapshot()
    })

    test('Typos', async () => {
        const hits = await suggest.search('helo')
        expect(hits).toMatchSnapshot()
    })

    test('Typos', async () => {
        const hits = await suggest.search('helo')
        expect(hits).toMatchSnapshot()
    })

    test('Joined words', async () => {
        const hits = await suggest.search('foobar')
        expect(hits).toMatchSnapshot()
    })

    test('Limit', async () => {
        const hits1 = await suggest.search('b')
        expect(hits1).toHaveLength(2)

        await suggest.setLimit(1)
        const hits2 = await suggest.search('b')
        expect(hits2).toHaveLength(1)

        await suggest.setLimit(100)
        const hits3 = await suggest.search('b')
        expect(hits3).toHaveLength(2)
    })

    test('Prio', async () => {
        const suggest = new LucidSuggest()
        suggest.setRecords(records.map((r, i) => ({...r, rating: i})))
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    describe('Render', () => {
        test('highlight helper', async () => {
            const suggest = new LucidSuggest()
            suggest.setRecords(records)
            const hits = await suggest.search('hel')
            const rendered = hits.map(hit => highlight(hit, '((', '))'))
            expect(rendered).toMatchSnapshot()
        })

        test('DIY', async () => {
            const suggest = new LucidSuggest()
            suggest.setRecords(records)
            const hits = await suggest.search('ba')
            const rendered = hits.map(hit => {
                return {
                    value: hit.record.id.toString(),
                    label: hit.chunks
                        .map(c => c.highlight ? '<<' + c.text + '>>' : c.text)
                        .join('')
                }
            })
            expect(rendered).toMatchSnapshot()
        })
    })
})