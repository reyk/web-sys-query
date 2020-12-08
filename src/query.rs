//! The main `Query` interface.

use crate::{selectors::Selectors, Error};
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    fmt,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCollection, HtmlElement};

/// Document with jQuery-like methods.
#[derive(AsRef, Clone, Debug, Deref, DerefMut, From, Into)]
pub struct Document(web_sys::Document);

impl Document {
    pub fn new() -> Result<Self, Error> {
        let inner = web_sys::window()
            .ok_or(Error::DomElementNotFound("window"))?
            .document()
            .ok_or(Error::DomElementNotFound("document"))?;

        Ok(Self(inner))
    }

    pub fn find(&self, selectors: &str) -> Result<Collection, Error> {
        let selectors = Selectors::new(selectors)?;
        Ok(selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into())
    }

    pub fn children(&self) -> Collection {
        self.0.children().into()
    }

    pub fn descendants(&self) -> Collection {
        if let Some(root) = self.0.document_element() {
            return Element::from(root).descendants();
        }
        Collection::new()
    }
}

/// Element with jQuery-like methods.
#[derive(AsRef, Clone, Deref, DerefMut, From, Into)]
pub struct Element(web_sys::Element);

impl Element {
    /// Set inner text.
    pub fn text(&self, text: &str) -> Result<(), Error> {
        self.dyn_ref::<web_sys::HtmlElement>()?.set_inner_text(text);
        Ok(())
    }

    /// Find elements by selector.
    pub fn find(&self, selectors: &str) -> Result<Collection, Error> {
        let selectors = Selectors::new(selectors)?;
        Ok(selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into())
    }

    /// Add CSS class.
    pub fn add_class(&self, class: &str) -> Result<(), Error> {
        self.0.class_list().add_1(class).map_err(Into::into)
    }

    /// Check if element has a matching CSS class.
    pub fn has_class(&self, class: &str) -> bool {
        self.0.class_list().contains(class)
    }

    /// Remove CSS class from list.
    pub fn remove_class(&self, class: &str) -> Result<(), Error> {
        self.0.class_list().add_1(class).map_err(Into::into)
    }

    /// Return the matching attribute, if found.
    pub fn attr(&self, key: &str) -> Option<String> {
        if key == "id" {
            return Some(self.0.id());
        }

        self.0.get_attribute(key)
    }

    /// Set the attribute to the specified value.
    pub fn set_attr(&self, key: &str, value: &str) -> Result<(), Error> {
        if key == "id" {
            self.0.set_id(value);
            return Ok(());
        }

        self.0.set_attribute(key, value).map_err(Into::into)
    }

    /// Remove the attribute.
    pub fn remove_attr(&self, key: &str) -> Result<(), Error> {
        if key == "id" {
            return Err(Error::CannotRemoveAttribute(self.0.id()));
        }

        self.0.remove_attribute(key).map_err(Into::into)
    }

    pub fn parent(&self) -> Option<Self> {
        self.parent_element().map(Into::into)
    }

    pub fn prev(&self) -> Option<Self> {
        self.previous_element_sibling().map(Into::into)
    }

    pub fn children(&self) -> Collection {
        self.0.children().into()
    }

    pub fn descendants(&self) -> Collection {
        let mut result = vec![];
        let mut nodes = vec![self.clone()];
        while let Some(node) = nodes.pop() {
            result.push(node.clone());
            for child in node.children().into_iter() {
                nodes.push(child);
            }
        }
        result.into()
    }

    pub fn next(&self) -> Option<Self> {
        self.next_element_sibling().map(Into::into)
    }

    pub fn dyn_ref<T: JsCast>(&self) -> Result<&T, Error> {
        self.0.dyn_ref::<T>().ok_or(Error::DynRefFailed)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.local_name())?;
        let id = self.id();
        if !id.is_empty() {
            write!(f, "[id=\"{}\"]", id)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)?;
        write!(f, "(children={}", self.child_element_count())?;
        let parent = self.parent_node().map(|elem| elem.node_type()).unwrap_or(0);
        write!(f, ", parent={})", parent)?;
        Ok(())
    }
}

impl<'a> TryInto<&'a web_sys::HtmlElement> for &'a Element {
    type Error = Error;

    fn try_into(self) -> Result<&'a web_sys::HtmlElement, Self::Error> {
        self.0.dyn_ref::<HtmlElement>().ok_or(Error::NotHtmlElement)
    }
}

impl TryFrom<&Document> for Element {
    type Error = Error;

    fn try_from(document: &Document) -> Result<Element, Self::Error> {
        document
            .0
            .document_element()
            .map(Into::into)
            .ok_or(Error::NoDocumentElement)
    }
}

/// HTML `Collection` that can be used as an iterator
#[derive(AsRef, Clone, Debug, Default, Deref, DerefMut)]
pub struct Collection(pub VecDeque<Element>);

impl Collection {
    pub fn new() -> Self {
        Default::default()
    }

    /// Move all elements of another collection into this collection.
    pub fn append_collection(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

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

    pub fn attr(&self, key: &str) -> Vec<String> {
        self.0.iter().filter_map(|elem| elem.attr(key)).collect()
    }

    pub fn set_attr(&self, key: &str, value: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.set_attr(key, value)?;
        }

        Ok(())
    }

    pub fn remove_attr(&self, key: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.remove_attr(key)?;
        }

        Ok(())
    }

    pub fn children(&self) -> Collection {
        let mut all_children = Collection::new();
        for element in self.0.iter() {
            all_children.append_collection(element.children());
        }
        all_children
    }
}

impl IntoIterator for Collection {
    type Item = Element;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
