import {Lang} from './lang'
import {EnglishStemmer} from './stemming/ext/EnglishStemmer'
import {CHAR_CLASSES_LATIN} from './constants'
import {CharClass} from './cls'
import {
    PartOfSpeech,
    ARTICLE,
    PREPOSITION,
    CONJUNCTION,
    PARTICLE,
} from './pos'


const FUNCTION_WORDS: [PartOfSpeech, string][] = [
    [ARTICLE, "a"],
    [ARTICLE, "an"],
    [ARTICLE, "the"],

    [PREPOSITION, "at"],
    [PREPOSITION, "by"],
    [PREPOSITION, "for"],
    [PREPOSITION, "from"],
    [PREPOSITION, "in"],
    [PREPOSITION, "of"],
    [PREPOSITION, "on"],
    [PREPOSITION, "to"],
    [PREPOSITION, "since"],
    [PREPOSITION, "before"],
    [PREPOSITION, "till"],
    [PREPOSITION, "untill"],
    [PREPOSITION, "beside"],
    [PREPOSITION, "under"],
    [PREPOSITION, "below"],
    [PREPOSITION, "over"],
    [PREPOSITION, "above"],
    [PREPOSITION, "across"],
    [PREPOSITION, "through"],
    [PREPOSITION, "into"],
    [PREPOSITION, "towards"],
    [PREPOSITION, "onto"],
    [PREPOSITION, "off"],
    [PREPOSITION, "out"],
    [PREPOSITION, "about"],

    [CONJUNCTION, "after"],
    [CONJUNCTION, "although"],
    [CONJUNCTION, "as"],
    [CONJUNCTION, "because"],
    [CONJUNCTION, "before"],
    [CONJUNCTION, "but"],
    [CONJUNCTION, "either"],
    [CONJUNCTION, "for"],
    [CONJUNCTION, "how"],
    [CONJUNCTION, "if"],
    [CONJUNCTION, "lest"],
    [CONJUNCTION, "nor"],
    [CONJUNCTION, "once"],
    [CONJUNCTION, "once"],
    [CONJUNCTION, "or"],
    [CONJUNCTION, "since"],
    [CONJUNCTION, "since"],
    [CONJUNCTION, "since"],
    [CONJUNCTION, "so"],
    [CONJUNCTION, "than"],
    [CONJUNCTION, "that"],
    [CONJUNCTION, "that"],
    [CONJUNCTION, "though"],
    [CONJUNCTION, "till"],
    [CONJUNCTION, "unless"],
    [CONJUNCTION, "until"],
    [CONJUNCTION, "until"],
    [CONJUNCTION, "until"],
    [CONJUNCTION, "what"],
    [CONJUNCTION, "whatever"],
    [CONJUNCTION, "when"],
    [CONJUNCTION, "when"],
    [CONJUNCTION, "whenever"],
    [CONJUNCTION, "where"],
    [CONJUNCTION, "whereas"],
    [CONJUNCTION, "whereas"],
    [CONJUNCTION, "wherever"],
    [CONJUNCTION, "whether"],
    [CONJUNCTION, "which"],
    [CONJUNCTION, "whichever"],
    [CONJUNCTION, "while"],
    [CONJUNCTION, "while"],
    [CONJUNCTION, "while"],
    [CONJUNCTION, "whilst"],
    [CONJUNCTION, "who"],
    [CONJUNCTION, "whoever"],
    [CONJUNCTION, "whom"],
    [CONJUNCTION, "whomever"],
    [CONJUNCTION, "whose"],
    [CONJUNCTION, "why"],
    [CONJUNCTION, "yet"],
    [CONJUNCTION, "and"],
    // [CONJUNCTION, "as if"],
    // [CONJUNCTION, "as long as"],
    // [CONJUNCTION, "as much as"],
    // [CONJUNCTION, "as soon as"],
    // [CONJUNCTION, "as though"],
    // [CONJUNCTION, "assuming that"],
    // [CONJUNCTION, "by the time"],
    // [CONJUNCTION, "even if"],
    // [CONJUNCTION, "even though"],
    // [CONJUNCTION, "in case that"],
    // [CONJUNCTION, "in case"],
    // [CONJUNCTION, "in order that"],
    // [CONJUNCTION, "in order"],
    // [CONJUNCTION, "now that"],
    // [CONJUNCTION, "only if"],
    // [CONJUNCTION, "provided that"],
    // [CONJUNCTION, "rather than"],
    // [CONJUNCTION, "so that"],

    [PARTICLE, "by"],
    [PARTICLE, "in"],
    [PARTICLE, "not"],
    [PARTICLE, "on"],
    [PARTICLE, "to"],
    [PARTICLE, "oh"],
]


const CHAR_CLASSES: [CharClass, string][] = [
]


const UTF_COMPOSE_MAP: [string, string][] = [
]


const UTF_REDUCE_MAP: [string, string][] = [
]


export const LANG_ENGLISH = new Lang(
    new EnglishStemmer(),
    [...CHAR_CLASSES_LATIN, ...CHAR_CLASSES],
    FUNCTION_WORDS,
    UTF_COMPOSE_MAP,
    UTF_REDUCE_MAP,
)
