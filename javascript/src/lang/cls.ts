
export const ANY         = 0
export const CONTROL     = 1
export const WHITESPACE  = 2
export const PUNCTUATION = 3
export const NOTALPHA    = 4
export const NOTALPHANUM = 5
export const CONSONANT   = 6
export const VOWEL       = 7


export type CharClass =
    | typeof ANY
    | typeof CONTROL
    | typeof WHITESPACE
    | typeof PUNCTUATION
    | typeof NOTALPHA
    | typeof NOTALPHANUM
    | typeof CONSONANT
    | typeof VOWEL

