//! jQuery-like API for web-sys
//!
//! See [here](https://github.com/reyk/web-sys-query/blob/main/README.md)
//! for more information.

mod error;
mod query;
mod selectors;

pub use crate::{
    error::Error,
    query::{Event, Collection, Document, Element},
    selectors::Selectors,
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
    ($obj:ident, $selector:expr) => {
        match AsRef::<str>::as_ref(&$selector).parse() {
            Ok(selectors) => Ok($obj.find(&selectors)),
            Err(err) => Err(err),
        }
    };
    ($selector:expr) => {
        match web_sys_query::Document::new() {
            Ok(document) => query!(document, $selector),
            Err(err) => Err(err),
        }
    };
}
