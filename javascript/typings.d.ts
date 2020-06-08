declare module 'lucid-suggest' {
    export interface Record {
        id: number,
        title: string,
        rating?: number,
    }

    export interface ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}

declare module 'lucid-suggest/en' {
    import {ILucidSuggest, Record} from 'lucid-suggest';
    export {Record} from 'lucid-suggest';
    export default class LucidSuggest implements ILucidSuggest {
        setRecords(records: Record[]): Promise<void>;
        highlightWith(left: string, right: string): Promise<void>;
        search(query: string): Promise<Record[]>;
    }
}