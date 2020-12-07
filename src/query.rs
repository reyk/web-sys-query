//! The main `Query` interface.

use crate::Error;
use derive_more::{AsRef, Deref, DerefMut};
use std::{collections::VecDeque, convert::TryInto};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCollection, HtmlElement};

/// Document with jQuery-like methods.
#[derive(AsRef, Clone, Deref, DerefMut)]
pub struct Document(web_sys::Document);

impl Document {
    pub fn new() -> Result<Self, Error> {
        let inner = web_sys::window()
            .ok_or(Error::DomElementNotFound("window"))?
            .document()
            .ok_or(Error::DomElementNotFound("document"))?;

        Ok(Self(inner))
    }

    pub fn find(&self, selector: &str) -> Collection {
        if let Some(selector) = selector.strip_prefix('#') {
            self.0.get_element_by_id(&selector).into()
        } else if let Some(selector) = selector.strip_prefix('.') {
            self.0.get_elements_by_class_name(&selector).into()
        } else {
            self.0.get_elements_by_tag_name(selector).into()
        }
    }
}

/// Element with jQuery-like methods.
#[derive(AsRef, Clone, Deref, DerefMut)]
pub struct Element(web_sys::Element);

impl Element {
    pub fn text(&self, text: &str) -> Result<(), Error> {
        self.dyn_ref::<web_sys::HtmlElement>()?.set_inner_text(text);
        Ok(())
    }

    pub fn find(&self, selector: &str) -> Collection {
        if selector.starts_with('#') {
            vec![].into()
        } else if let Some(selector) = selector.strip_prefix('.') {
            self.0.get_elements_by_class_name(&selector).into()
        } else {
            self.0.get_elements_by_tag_name(selector).into()
        }
    }

    pub fn add_class(&self, class: &str) -> Result<(), Error> {
        self.0.class_list().add_1(class).map_err(Into::into)
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.0.class_list().contains(class)
    }

    pub fn remove_class(&self, class: &str) -> Result<(), Error> {
        self.0.class_list().add_1(class).map_err(Into::into)
    }

    pub fn attr(&self, key: &str, value: &str) -> Result<(), Error> {
        self.0.set_attribute(key, value).map_err(Into::into)
    }

    pub fn remove_attr(&self, key: &str) -> Result<(), Error> {
        self.0.remove_attribute(key).map_err(Into::into)
    }

    pub fn dyn_ref<T: JsCast>(&self) -> Result<&T, Error> {
        self.0.dyn_ref::<T>().ok_or(Error::DynRefFailed)
    }
}

impl From<web_sys::Element> for Element {
    fn from(inner: web_sys::Element) -> Self {
        Self(inner)
    }
}

impl<'a> TryInto<&'a web_sys::HtmlElement> for &'a Element {
    type Error = Error;

    fn try_into(self) -> Result<&'a web_sys::HtmlElement, Self::Error> {
        self.0.dyn_ref::<HtmlElement>().ok_or(Error::NotHtmlElement)
    }
}

/// HTML `Collection` that can be used as an iterator
#[derive(AsRef, Clone, Deref, DerefMut)]
pub struct Collection(VecDeque<Element>);

impl Collection {
    pub fn first(&mut self) -> Result<Element, Error> {
        self.0.pop_front().ok_or(Error::FirstElementNotFound)
    }

    pub fn text(&self, text: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.text(text)?;
        }

        Ok(())
    }

    pub fn add_class(&self, class: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.class_list().add_1(class)?;
        }

        Ok(())
    }

    pub fn has_class(&self, class: &str) -> bool {
        for element in self.0.iter() {
            if element.class_list().contains(class) {
                return true;
            }
        }

        false
    }

    pub fn remove_class(&self, class: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.class_list().remove_1(class)?;
        }

        Ok(())
    }

    pub fn attr(&self, key: &str, value: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.attr(key, value)?;
        }

        Ok(())
    }

    pub fn remove_attr(&self, key: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.remove_attr(key)?;
        }

        Ok(())
    }
}

impl From<HtmlCollection> for Collection {
    fn from(collection: HtmlCollection) -> Self {
        let mut inner = VecDeque::new();

        for i in 0..collection.length() {
            if let Some(item) = collection.item(i) {
                inner.push_back(item.into());
            }
        }

        Self(inner)
    }
}

impl From<Vec<Element>> for Collection {
    fn from(collection: Vec<Element>) -> Self {
        Self(collection.into())
    }
}

impl From<Option<web_sys::Element>> for Collection {
    fn from(element: Option<web_sys::Element>) -> Self {
        let inner = match element {
            Some(element) => vec![element.into()],
            None => vec![],
        };

        Self(inner.into())
    }
}
