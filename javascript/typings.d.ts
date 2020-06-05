export type Lang =
    | 'de'
    | 'en'
    | 'es'
    | 'pt'
    | 'ru';

export type Record = {
    id: number,
    title: string,
    rating?: number,
};

declare class LucidSuggest {
    setLanguage(lang: Lang): Promise<void>;
    setRecords(records: Record[]): Promise<void>;
    highlightWith(left: string, right: string): Promise<void>;
    search(query: string): Promise<Record[]>;
}

export default LucidSuggest;
