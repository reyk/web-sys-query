use derive_more::From;
use displaydoc::Display;
use thiserror::Error;
use wasm_bindgen::JsValue;
use crate::Event;

#[derive(Debug, Display, Error, From)]
pub enum Error {
    /// Cannot remove attribute: '{0}'
    #[from(ignore)]
    CannotRemoveAttribute(String),
    /// Child element not found: '{0}'
    #[from(ignore)]
    ChildElementNotFound(&'static str),
    /// DOM element not found: '{0}'
    #[from(ignore)]
    DomElementNotFound(&'static str),
    /// Failed to get dynamic reference type
    DynRefFailed,
    /// Element ID not found: '{0}'
    #[from(ignore)]
    ElementIdNotFound(&'static str),
    /// Event not implemented: `{0:?}˙
    EventNotImplemented(Event),
    /// Event not handled: `{0:?}˙
    #[from(ignore)]
    EventNotHandled(Event),
    /// First element in collection not found
    FirstElementNotFound,
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
