
export interface Stemmer {
    setCurrent(word: string): void;
    getCurrent(): string;
    stem(): boolean;
}
