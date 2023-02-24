pub mod que;
pub mod store;

pub use que::*;
pub use store::*;

pub mod ans;
pub use ans::*;


#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}
