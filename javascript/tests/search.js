const mod = require('../build/index')


describe('Suggest', () => {
    beforeAll(() => {
        mod.storeRecords([
            {id: 10, text: 'Hello world!'},
            {id: 20, text: 'Foo bar'},
            {id: 30, text: '-BAZZZ-'},
        ])
    })
    
    test('Empty input', () => {
        const hits = mod.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', () => {
        const hits = mod.search('foo bar')
        expect(hits).toMatchSnapshot()
    })

    test('Prefix', () => {
        const hits = mod.search('ba')
        expect(hits).toMatchSnapshot()
    })
})