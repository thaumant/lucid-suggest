import {LangAbstract} from './lang'
import {Stemmer} from './stemming/ext/PlaceholderStemmer'

export class Lang extends LangAbstract {
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
