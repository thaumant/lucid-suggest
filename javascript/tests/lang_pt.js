const {LucidSuggest} = require('../pt')


describe('Portuguese language', () => {
    const records = [
        {id: 10, title: 'Pilhas Alcalinas AA — Embalagem com 12'},
        {id: 20, title: 'Cabo Lightning para USB-A'},
        {id: 30, title: 'Escova de dentes elétrica'},
        {id: 40, title: 'Sacos de armazenamento a vácuo'},
    ]

    const suggest = new LucidSuggest()
    suggest.setRecords(records)

    test('Empty input', async () => {
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', async () => {
        const hits = await suggest.search('escova de dentes eletrica')
        expect(hits).toMatchSnapshot()
    })

    test('Stemming', async () => {
        const hits = await suggest.search('alcalino')
        expect(hits).toMatchSnapshot()
    })

    test('Partiles', async () => {
        const hits = await suggest.search('de')
        expect(hits).toMatchSnapshot()
    })

    test('Normalization', async () => {
        const hits = await suggest.search('electric')
        expect(hits).toMatchSnapshot()
    })
})