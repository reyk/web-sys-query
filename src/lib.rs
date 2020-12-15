//! jQuery-like API for web-sys
//!
//! See [here](https://github.com/reyk/web-sys-query/blob/main/README.md)
//! for more information.

mod error;
mod query;

pub use crate::{
    error::Error,
    query::{Collection, Document, Element, Event, FormData},
};

/// `query!` macro to find elements.
///
/// This macro is a helper for the `find` method.
///
/// # Examples
///
/// ```rust
/// use wasm_bindgen::prelude::*;
/// use web_sys_query::{query, Error};
///
/// #[wasm_bindgen]
/// pub fn hello() {
///     query!("body").unwrap().set_text("Hello, World!");
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! query {
    ($obj:expr, $selectors:expr) => {
        $obj.find($selectors)
    };
    ($selectors:expr) => {
        match web_sys_query::Document::new() {
            Ok(document) => query!(document, $selectors),
            Err(err) => Err(err),
        }
    };
}
