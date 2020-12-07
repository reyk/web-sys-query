//! Minimal jQuery-like API for web-sys

mod error;
mod query;

pub use error::Error;
pub use query::{Collection, Document, Element};
