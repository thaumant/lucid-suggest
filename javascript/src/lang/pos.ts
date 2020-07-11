
export const UNKNOWN     = 0
export const NOUN        = 1
export const PRONOUN     = 2
export const VERB        = 3
export const ADJECTIVE   = 4
export const ADVERB      = 5
export const PREPOSITION = 6
export const CONJUNCTION = 7
export const PARTICLE    = 8
export const INTEJECTION = 9
export const ARTICLE     = 10

export type PartOfSpeech =
    | typeof UNKNOWN
    | typeof NOUN
    | typeof PRONOUN
    | typeof VERB
    | typeof ADJECTIVE
    | typeof ADVERB
    | typeof PREPOSITION
    | typeof CONJUNCTION
    | typeof PARTICLE
    | typeof INTEJECTION
    | typeof ARTICLE
