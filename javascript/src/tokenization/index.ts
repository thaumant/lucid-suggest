import type {LangAbstract} from '../lang/lang'
import {WHITESPACE, CONTROL, PUNCTUATION, NOTALPHANUM} from '../lang/cls'
import {Text} from './text'


export function tokenizeQuery(source: string, lang: LangAbstract): Text {
    return (new Text(source))
        .normalize(lang)
        .setFin(false)
        .split([WHITESPACE, CONTROL, PUNCTUATION], lang)
        .strip([NOTALPHANUM], lang)
        .lower()
        .setPos(lang)
        .setCharClasses(lang)
        .setStem(lang)
}


export function tokenizeRecord(source: string, lang: LangAbstract): Text {
    return (new Text(source))
        .normalize(lang)
        .split([WHITESPACE, CONTROL, PUNCTUATION], lang)
        .strip([NOTALPHANUM], lang)
        .lower()
        .setPos(lang)
        .setCharClasses(lang)
        .setStem(lang)
}
