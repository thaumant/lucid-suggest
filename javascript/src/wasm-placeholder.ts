
export interface WasmAPI {
    create_store(): number;
    destroy_store(store_id: number): void;
    set_limit(store_id: number, limit: number): void;
    add_record(
        store_id:  number,
        record_id: number,
        rating:    number,
        source:    string,
        chars:     string,
        classes:   Uint32Array,
        words:     Uint32Array,
    ): void;
    run_search(
        store_id: number,
        source:   string,
        chars:    string,
        classes:  Uint32Array,
        words:    Uint32Array,
    ): number;
    get_result_ids(store_id: number): number[];
    get_result_titles(store_id: number): string;
}

