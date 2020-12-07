use displaydoc::Display;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Display, Error)]
pub enum Error {
    /// Child element not found: '{0}'
    ChildElementNotFound(&'static str),
    /// DOM element not found: '{0}'
    DomElementNotFound(&'static str),
    /// Element ID not found: '{0}'
    ElementIdNotFound(&'static str),
    /// Element not found
    ElementNotFound,
    /// Failed to case to HTML element
    NotHtmlElement,
    /// Other Error
    JsValue(JsValue),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        match self {
            Error::JsValue(value) => value,
            _ => JsValue::from_str(&self.to_string()),
        }
    }
}
