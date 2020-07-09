import * as readline from 'readline'
import {LucidSuggest, Hit} from 'lucid-suggest/en'
import DATA from './e_commerce.json'

const CODE_RESET = '\x1b[0m'
const CODE_RED   = '\x1b[31m'
const CODE_BOLD  = '\x1b[1m'

const suggest = new LucidSuggest
suggest.setRecords(DATA)

function render(hit: Hit): string {
    return hit.chunks
        .map(chunk => {
            return chunk.highlight
                ? CODE_BOLD + CODE_RED + chunk.text + CODE_RESET
                : chunk.text
        })
        .join('')
}

const rl = readline.createInterface({
    input:  process.stdin,
    output: process.stdout,
})

function ready() {
    rl.question('> ', async query => {
        const start = Date.now()
        const hits  = await suggest.search(query)
        console.log(Date.now() - start, 'ms')
        console.log('--------------------------------')
        for (const hit of hits) {
            console.log(render(hit))
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
