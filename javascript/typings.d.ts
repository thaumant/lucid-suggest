declare module 'lucid-suggest' {
    export interface Record {
        id: number,
        title: string,
        rating?: number,
    }

    export interface ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        setLimit(limit: number): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}


declare module 'lucid-suggest/de' {
    import {ILucidSuggest, Record} from 'lucid-suggest';
    export {Record} from 'lucid-suggest';
    export class LucidSuggest implements ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        setLimit(limit: number): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}


declare module 'lucid-suggest/en' {
    import {ILucidSuggest, Record} from 'lucid-suggest';
    export {Record} from 'lucid-suggest';
    export class LucidSuggest implements ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        setLimit(limit: number): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}


declare module 'lucid-suggest/es' {
    import {ILucidSuggest, Record} from 'lucid-suggest';
    export {Record} from 'lucid-suggest';
    export class LucidSuggest implements ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        setLimit(limit: number): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}


declare module 'lucid-suggest/pt' {
    import {ILucidSuggest, Record} from 'lucid-suggest';
    export {Record} from 'lucid-suggest';
    export class LucidSuggest implements ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        setLimit(limit: number): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}


declare module 'lucid-suggest/ru' {
    import {ILucidSuggest, Record} from 'lucid-suggest';
    export {Record} from 'lucid-suggest';
    export class LucidSuggest implements ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        setLimit(limit: number): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}
