const {LucidSuggest} = require('../en')


describe('English language', () => {
    const records = [
        {id: 10, title: 'AA 1.5 Alkaline Batteries — Pack of 12'},
        {id: 20, title: 'Lightning to USB A Cable'},
        {id: 30, title: 'Electric Toothbrush'},
        {id: 40, title: 'Vacuum Compression Storage Bags'},
    ]

    const suggest = new LucidSuggest()
    suggest.setRecords(records)

    test('Empty input', async () => {
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', async () => {
        const hits = await suggest.search('electric toothbrush')
        expect(hits).toMatchSnapshot()
    })

    test('Stemming', async () => {
        const hits = await suggest.search('battery aa')
        expect(hits).toMatchSnapshot()
    })

    test('Partiles', async () => {
        const hits = await suggest.search('to')
        expect(hits).toMatchSnapshot()
    })
})