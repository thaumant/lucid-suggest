const parse = require('csv-parse/lib/sync')
const fs = require('fs')

const records = parse(fs.readFileSync('./e_commerce_source.csv', 'utf8'), {
    columns:          true,
    trim:             true,
    skip_empty_lines: true,
    cast(val, {column}) {
        if (column == 'StockCode')   return parseInt(val)
        if (column == 'InvoiceNo')   return parseInt(val)
        if (column == 'Quantity')    return parseInt(val)
        if (column == 'UnitPrice')   return parseFloat(val)
        if (column == 'Description') return val.charAt(0).toUpperCase() + val.slice(1).toLowerCase()
        return val
    }
})


const indexed = new Map()
for (const {StockCode, Description, Quantity} of records) {
    if (!StockCode || !Description) continue;
    if (!(Quantity > 0)) continue;

    const stat = indexed.get(StockCode)
    if (!stat) {
        indexed.set(StockCode, {
            id:     StockCode,
            title:  Description,
            rating: Quantity,
        })
    } else {
        stat.rating += Quantity
    }
}

const output = Array.from(indexed.values())
output.sort((r1, r2) => r2.rating - r1.rating)


// const fd = fs.openSync('./e_commerce.json', 'w')

// fs.writeSync(fd, '[\n')
// for (const record of output) {
//     fs.writeSync(fd, '    ')
//     fs.writeSync(fd, JSON.stringify(record))
//     fs.writeSync(fd, ',\n')
// }
// fs.writeSync(fd, ']\n')


fs.writeFileSync('./e_commerce.json', JSON.stringify(output, null, 4))
