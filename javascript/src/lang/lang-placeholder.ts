import {Lang} from './lang'
import {PlaceholderStemmer} from './stemming/ext/PlaceholderStemmer'

export class LangPlaceholder extends Lang {
    constructor() {
        super(
            new PlaceholderStemmer(),
            [],
            [],
            [],
            [],
        )
    }
}
