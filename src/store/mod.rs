mod error;
mod store;
mod store_mem;
mod store_sqlite;

pub use error::Error;
pub use store::Store;
pub use store_mem::StoreMem;
pub use store_sqlite::StoreSqlite;

use error::Result;
