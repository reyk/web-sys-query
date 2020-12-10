use derive_more::From;
use displaydoc::Display;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Display, Error, From)]
pub enum Error {
    /// Child element not found: '{0}'
    #[from(ignore)]
    ChildElementNotFound(&'static str),
    /// DOM element not found: '{0}'
    #[from(ignore)]
    DomElementNotFound(&'static str),
    /// Element ID not found: '{0}'
    #[from(ignore)]
    ElementIdNotFound(&'static str),
    /// First element in collection not found
    FirstElementNotFound,
    /// Failed to get dynamic reference type
    DynRefFailed,
    /// Failed to cast to HTML element
    NotHtmlElement,
    /// Failed to cast to `EventTarget`
    NotEventTarget,
    /// `Document` without document `Element`
    NoDocumentElement,
    /// `Event` does not have a target element
    NoTargetElement,
    /// `Element` type does not have '{0}' value
    #[from(ignore)]
    NoValue(&'static str),
    /// Cannot remote attribute: '{0}'
    #[from(ignore)]
    CannotRemoveAttribute(String),
    /// Selectors Parser Error
    SelectorsParserError,
    /// Other Error
    JsValue(JsValue),
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        match self {
            Error::JsValue(value) => value,
            _ => JsValue::from_str(&self.to_string()),
        }
    }
}
