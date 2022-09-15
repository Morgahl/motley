pub mod mock;

mod error;
mod generator;
mod table;

pub use generator::{AutoIncrement, UUIDv4};
pub use table::Table;

pub use error::*;
