import {LangAbstract} from './lang'
import {Stemmer} from './stemming/ext/GermanStemmer'
import {CHAR_CLASSES_LATIN} from './constants'
import {CharClass, CONSONANT} from './cls'
import {
    PartOfSpeech,
    ARTICLE,
    PREPOSITION,
    CONJUNCTION,
    PARTICLE,
} from './pos'


const FUNCTION_WORDS: [PartOfSpeech, string][] = [
    [ARTICLE, "das"],
    [ARTICLE, "dem"],
    [ARTICLE, "den"],
    [ARTICLE, "der"],
    [ARTICLE, "des"],
    [ARTICLE, "die"],
    [ARTICLE, "ein"],
    [ARTICLE, "eine"],
    [ARTICLE, "einem"],
    [ARTICLE, "einen"],
    [ARTICLE, "einer"],
    [ARTICLE, "eines"],

    [PREPOSITION, "an"],
    [PREPOSITION, "auf"],
    [PREPOSITION, "aus"],
    [PREPOSITION, "bei"],
    [PREPOSITION, "bis"],
    [PREPOSITION, "durch"],
    [PREPOSITION, "entlang"],
    [PREPOSITION, "für"],
    [PREPOSITION, "gegen"],
    [PREPOSITION, "hinter"],
    [PREPOSITION, "in"],
    [PREPOSITION, "mit"],
    [PREPOSITION, "nach"],
    [PREPOSITION, "neben"],
    [PREPOSITION, "ohne"],
    [PREPOSITION, "seit"],
    [PREPOSITION, "um"],
    [PREPOSITION, "von"],
    [PREPOSITION, "zu"],

    [CONJUNCTION, "aber"],
    [CONJUNCTION, "als"],
    [CONJUNCTION, "als"],
    [CONJUNCTION, "anstatt"],
    [CONJUNCTION, "auch"],
    [CONJUNCTION, "bevor"],
    [CONJUNCTION, "bis"],
    [CONJUNCTION, "but"],
    [CONJUNCTION, "damit"],
    [CONJUNCTION, "dass"],
    [CONJUNCTION, "denn"],
    [CONJUNCTION, "entweder"],
    [CONJUNCTION, "nachdem"],
    [CONJUNCTION, "noch"],
    [CONJUNCTION, "ob"],
    [CONJUNCTION, "obwohl"],
    [CONJUNCTION, "oder"],
    [CONJUNCTION, "oder"],
    [CONJUNCTION, "seitdem"],
    [CONJUNCTION, "sobald"],
    [CONJUNCTION, "sofern"],
    [CONJUNCTION, "sondern"],
    [CONJUNCTION, "soweit"],
    [CONJUNCTION, "sowie"],
    [CONJUNCTION, "sowohl"],
    [CONJUNCTION, "sowohl"],
    [CONJUNCTION, "the"],
    [CONJUNCTION, "und"],
    [CONJUNCTION, "während"],
    [CONJUNCTION, "weder"],
    [CONJUNCTION, "weil"],
    [CONJUNCTION, "wenn"],
    [CONJUNCTION, "wie"],
    [CONJUNCTION, "wie"],
    [CONJUNCTION, "wo"],
    [CONJUNCTION, "zu"],

    [PARTICLE, "schon"],
    [PARTICLE, "ja"],
    [PARTICLE, "halt"],
    [PARTICLE, "wohl"],
    [PARTICLE, "doch"],
    [PARTICLE, "mal"],
    [PARTICLE, "aber"],
    [PARTICLE, "auch"],
    [PARTICLE, "bloß"],
    [PARTICLE, "denn"],
    [PARTICLE, "eben"],
    [PARTICLE, "etwas"],
    [PARTICLE, "nur"],
    [PARTICLE, "ruhig"],
    [PARTICLE, "shon"],
    [PARTICLE, "zwar"],
    [PARTICLE, "soweiso"],
]


const CHAR_CLASSES: [CharClass, string][] = [
    [CONSONANT, 'ß'],
]


const UTF_COMPOSE_MAP: [string, string][] = [
    ["Ä", "Ä"],
    ["Ö", "Ö"],
    ["Ü", "Ü"],
    ["ä", "ä"],
    ["ö", "ö"],
    ["ü", "ü"],
]


const UTF_REDUCE_MAP: [string, string][] = [
    ["ẞ", "SS"], // eszett
    ["ß", "ss"],
    ["Ä", "A"], // umlauts
    ["Ö", "O"],
    ["Ü", "U"],
    ["ä", "a"],
    ["ö", "o"],
    ["ü", "u"],
]


export class Lang extends LangAbstract {
    constructor() {
        super(
            new Stemmer(),
            [...CHAR_CLASSES_LATIN, ...CHAR_CLASSES],
            FUNCTION_WORDS,
            UTF_COMPOSE_MAP,
            UTF_REDUCE_MAP,
        )
    }
}
