import {Lang} from './lang'
import {SpanishStemmer} from './stemming/ext/SpanishStemmer'
import {CHAR_CLASSES_LATIN} from './constants'
import {CharClass} from './cls'
import {
    PartOfSpeech,
    ARTICLE,
    PREPOSITION,
    CONJUNCTION,
} from './pos'


const FUNCTION_WORDS: [PartOfSpeech, string][] = [
    [ARTICLE, "el"],
    [ARTICLE, "la"],
    [ARTICLE, "los"],
    [ARTICLE, "las"],
    [ARTICLE, "un"],
    [ARTICLE, "una"],
    [ARTICLE, "unos"],
    [ARTICLE, "unas"],

    [PREPOSITION, "a"],
    [PREPOSITION, "abajo"],
    [PREPOSITION, "alrededor"],
    [PREPOSITION, "antes"],
    [PREPOSITION, "aquellos"],
    [PREPOSITION, "arriba"],
    [PREPOSITION, "bajo"],
    [PREPOSITION, "como"],
    [PREPOSITION, "con"],
    [PREPOSITION, "contra"],
    [PREPOSITION, "de"],
    [PREPOSITION, "dentro"],
    [PREPOSITION, "desde"],
    [PREPOSITION, "durante"],
    [PREPOSITION, "en"],
    [PREPOSITION, "encima"],
    [PREPOSITION, "entre"],
    [PREPOSITION, "esta"],
    [PREPOSITION, "esto"],
    [PREPOSITION, "estos"],
    [PREPOSITION, "fuera"],
    [PREPOSITION, "hacia"],
    [PREPOSITION, "hasta"],
    [PREPOSITION, "más"],
    [PREPOSITION, "opuesto"],
    [PREPOSITION, "para"],
    [PREPOSITION, "pero"],
    [PREPOSITION, "por"],
    [PREPOSITION, "próximo"],
    [PREPOSITION, "que"],
    [PREPOSITION, "salvo"],
    [PREPOSITION, "sin"],
    [PREPOSITION, "sobre"],
    [PREPOSITION, "vía"],
    // [PREPOSITION, "a causa de"],
    // [PREPOSITION, "a diferencia de"],
    // [PREPOSITION, "a pesar de"],
    // [PREPOSITION, "a pesar de"],
    // [PREPOSITION, "a través de"],
    // [PREPOSITION, "a través de"],
    // [PREPOSITION, "además de"],
    // [PREPOSITION, "al lado de"],
    // [PREPOSITION, "al lado de"],
    // [PREPOSITION, "alrededor de"],
    // [PREPOSITION, "antes de"],
    // [PREPOSITION, "así como"],
    // [PREPOSITION, "cerca de"],
    // [PREPOSITION, "cerca de"],
    // [PREPOSITION, "cerca de"],
    // [PREPOSITION, "de acuerdo con"],
    // [PREPOSITION, "debido a"],
    // [PREPOSITION, "delante de"],
    // [PREPOSITION, "dentro de"],
    // [PREPOSITION, "dentro de"],
    // [PREPOSITION, "después de"],
    // [PREPOSITION, "detrás de"],
    // [PREPOSITION, "en lo que"],
    // [PREPOSITION, "en lugar de"],
    // [PREPOSITION, "en nombre de"],
    // [PREPOSITION, "encima de"],
    // [PREPOSITION, "fuera de"],
    // [PREPOSITION, "lejos de"],
    // [PREPOSITION, "más allá de"],
    // [PREPOSITION, "por debajo de"],
    // [PREPOSITION, "por encima"],
    // [PREPOSITION, "tres palabras"],

    [CONJUNCTION, "aunque"],
    [CONJUNCTION, "como"],
    [CONJUNCTION, "e"],
    [CONJUNCTION, "entonces"],
    [CONJUNCTION, "excepto"],
    [CONJUNCTION, "mas"],
    [CONJUNCTION, "o"],
    [CONJUNCTION, "pero"],
    [CONJUNCTION, "porque"],
    [CONJUNCTION, "pues"],
    [CONJUNCTION, "si"],
    [CONJUNCTION, "sino"],
    [CONJUNCTION, "u"],
    [CONJUNCTION, "y"],
    // [CONJUNCTION, "con tal de que"],
    // [CONJUNCTION, "es decir"],
    // [CONJUNCTION, "esto es"],
    // [CONJUNCTION, "ni … ni"],
    // [CONJUNCTION, "no obstante"],
    // [CONJUNCTION, "o … o"],
    // [CONJUNCTION, "o bien … o bien"],
    // [CONJUNCTION, "por lo demás"],
    // [CONJUNCTION, "puesto que"],
    // [CONJUNCTION, "sea … sea"],
    // [CONJUNCTION, "siempre que"],
    // [CONJUNCTION, "sin embargo"],
    // [CONJUNCTION, "ya que"],
]


const CHAR_CLASSES: [CharClass, string][] = [
]


const UTF_COMPOSE_MAP: [string, string][] = [
    ["Á", "Á"], // acute accent
    ["É", "É"],
    ["Í", "Í"],
    ["Ó", "Ó"],
    ["Ú", "Ú"],
    ["á", "á"],
    ["é", "é"],
    ["í", "í"],
    ["ó", "ó"],
    ["ú", "ú"],
    ["Ñ", "Ñ"], // tilde
    ["ñ", "ñ"],
    ["Ü", "Ü"], // diaeresis
    ["ü", "ü"],
]


const UTF_REDUCE_MAP: [string, string][] = [
    ["Á", "A"], // acute accent
    ["É", "E"],
    ["Í", "I"],
    ["Ó", "O"],
    ["Ú", "U"],
    ["á", "a"],
    ["é", "e"],
    ["í", "i"],
    ["ó", "o"],
    ["ú", "u"],
    ["Ñ", "N"], // tilde
    ["ñ", "n"],
    ["Ü", "U"], // diaeresis
    ["ü", "u"],
]


export const LANG_SPANISH = new Lang(
    new SpanishStemmer(),
    [...CHAR_CLASSES_LATIN, ...CHAR_CLASSES],
    FUNCTION_WORDS,
    UTF_COMPOSE_MAP,
    UTF_REDUCE_MAP,
)
