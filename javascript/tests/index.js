const LucidSuggest = require('../build/index')

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

    test('Prio', async () => {
        const suggest = new LucidSuggest()
        suggest.setRecords(records.map((r, i) => ({...r, rating: i})))
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })
})