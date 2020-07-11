import {Lang} from './lang'
import {Stemmer} from './stemming/ext/PlaceholderStemmer'

export class LangPlaceholder extends Lang {
    constructor() {
        super(
            new Stemmer(),
            [],
            [],
            [],
            [],
        )
    }
}
