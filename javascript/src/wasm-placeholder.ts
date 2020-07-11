
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


const compileWasm: Promise<WasmAPI> = Promise.resolve({
    create_store() { throw new Error("create_store not implemented") },
    destroy_store() { throw new Error("destroy_store not implemented") },
    set_limit() { throw new Error("set_limit not implemented") },
    add_record() { throw new Error("add_record not implemented") },
    run_search() { throw new Error("run_search not implemented") },
    get_result_ids() { throw new Error("get_result_ids not implemented") },
    get_result_titles() { throw new Error("get_result_titles not implemented") },
})

export default compileWasm
