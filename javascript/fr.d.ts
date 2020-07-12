export class LucidSuggest {
    addRecords(records: Record[]): Promise<void>;
    setLimit(limit: number): Promise<void>;
    search(query: string): Promise<Hit[]>;
}


export interface Record {
    id:      number,
    title:   string,
    rating?: number,
}


export class Hit {
    title:    string;
    chunks:   HighlightedTextChunk[];
    record:   Record;
    position: number;
}


export type HighlightedTextChunk = {
    text:      string,
    highlight: boolean,
}


export function highlight(hit: Hit, left: string, right: string): string;
