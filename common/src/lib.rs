pub mod card;
pub mod comparator;
pub mod ext;
pub mod lead;
pub mod macros;
pub mod message;
pub mod model;
pub mod phase;
pub mod serialize;
pub mod state;

pub type MyResult<T> = Result<T, String>;
