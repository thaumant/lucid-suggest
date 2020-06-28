import * as readline from 'readline'
import {LucidSuggest} from 'lucid-suggest/en'
import DATA from './e_commerce.json'

const CODE_RESET = '\x1b[0m'
const CODE_RED   = '\x1b[31m'
const CODE_GREEN = '\x1b[36m'
const CODE_BOLD  = '\x1b[1m'

const suggest = new LucidSuggest
suggest.highlightWith(CODE_BOLD + CODE_RED, CODE_RESET)
suggest.setRecords(DATA)

const rl = readline.createInterface({
    input:  process.stdin,
    output: process.stdout,
})

function ready() {
    rl.question('> ', async query => {
        const start   = Date.now()
        const results = await suggest.search(query)
        console.log(Date.now() - start, 'ms')
        console.log('--------------------------------')
        for (const result of results) {
            console.log(result.title)
        }
        console.log('--------------------------------')
        console.log('\n')
        ready()
    })
}

rl.on('close', function() {
    console.log('\n');
    process.exit(0);
});

console.log('Input query and press Enter.')
ready()
