//! Minimal jQuery-like API for web-sys

mod error;
mod query;
mod selectors;

pub use error::Error;
pub use query::{Collection, Document, Element};
