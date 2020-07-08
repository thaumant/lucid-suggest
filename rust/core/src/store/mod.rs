mod record;
mod store;
mod trigram_index;

pub use record::Record;
pub use store::Store;
pub use trigram_index::TrigramIndex;

pub static DEFAULT_LIMIT: usize = 10;
