const LucidSuggest = require('../build/index')


describe('Suggest', () => {
    const records = [
        {id: 10, title: 'Hello world!'},
        {id: 20, title: 'Foo bar'},
        {id: 30, title: '-BAZZZ-'},
    ]

    const suggest = new LucidSuggest()
    suggest.setRecords(records)

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