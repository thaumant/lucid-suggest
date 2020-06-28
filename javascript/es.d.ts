
export interface Record {
    id:      number,
    title:   string,
    rating?: number,
}

export class LucidSuggest {
    setRecords(records: Record[]): Promise<void>;
    setLimit(limit: number): Promise<void>;
    highlightWith(left: string, right: string): Promise<void>;
    search(query: string): Promise<Record[]>;
}
