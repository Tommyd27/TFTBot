/* Prelude */
//default imports of Error and Result
pub use crate::error::Error; 

pub type Result<T> = core::result::Result<T, Error>;