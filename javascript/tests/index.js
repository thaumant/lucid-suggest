const compile = require('../build/index')


describe('Suggest', () => {
    const records = [
        {id: 10, title: 'Hello world!'},
        {id: 20, title: 'Foo bar'},
        {id: 30, title: '-BAZZZ-'},
    ]

    let suggest = null

    beforeAll(() => {
        return compile.then(LucidSuggest => {
            suggest = new LucidSuggest()
            suggest.setRecords(records)
        })
    })

    test('Empty input', () => {
        const hits = suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', () => {
        const hits = suggest.search('foo bar')
        expect(hits).toMatchSnapshot()
    })

    test('Prefix', () => {
        const hits = suggest.search('ba')
        expect(hits).toMatchSnapshot()
    })

    test('Prio', () => {
        return compile.then(LucidSuggest => {
            suggest = new LucidSuggest()
            suggest.setRecords(records.map((r, i) => ({...r, rating: i})))
            const hits = suggest.search('')
            expect(hits).toMatchSnapshot()
        })
    })
})