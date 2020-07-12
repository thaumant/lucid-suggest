const {LucidSuggest} = require('../fr')


describe('French language', () => {
    const records = [
        {id: 10, title: 'Piles alcalines 1,5 V AA - Lot de 12'},
        {id: 20, title: 'Câble Lightning vers USB A'},
        {id: 30, title: 'Brosse À Dents Électrique'},
        {id: 40, title: 'Sacs de Rangement Sous Vide'},
    ]

    const suggest = new LucidSuggest()
    suggest.addRecords(records)

    test('Empty input', async () => {
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', async () => {
        const hits = await suggest.search('sacs de rangement sous vide')
        expect(hits).toMatchSnapshot()
    })

    test('Stemming', async () => {
        const hits = await suggest.search('pile aa')
        expect(hits).toMatchSnapshot()
    })

    test('Partiles', async () => {
        const hits = await suggest.search('de')
        expect(hits).toMatchSnapshot()
    })

    test('Normalization', async () => {
        const hits1 = await suggest.search('électrique')
        const hits2 = await suggest.search('electrique')
        expect(hits1).toMatchSnapshot()
        expect(hits2).toMatchSnapshot()
    })
})