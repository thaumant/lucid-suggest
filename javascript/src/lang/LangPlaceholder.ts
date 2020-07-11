import Lang from './Lang'
import Stemmer from './stemming/ext/PlaceholderStemmer'

export default class LangPlaceholder extends Lang {
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
