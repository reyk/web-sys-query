//! The main `Query` interface.

mod attributes;
mod events;
mod manipulation;
mod traversing;

use crate::Error;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    fmt,
    iter::FromIterator,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCollection, HtmlElement, NodeList};

pub use events::Event;

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

    pub fn descendants(&self) -> Collection {
        Element::try_from(self)
            .as_ref()
            .map(Element::descendants)
            .unwrap_or_default()
    }
}

/// Element with jQuery-like methods.
#[derive(AsRef, Clone, Deref, DerefMut, From, Into)]
pub struct Element(web_sys::Element);

impl Element {
    pub fn descendants(&self) -> Collection {
        let mut result = vec![];
        let mut nodes = vec![self.clone()];
        while let Some(node) = nodes.pop() {
            result.push(node.clone());
            for child in Collection::from(node.0.children()).into_iter().rev() {
                nodes.push(child);
            }
        }
        result.into()
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

impl TryFrom<Document> for Element {
    type Error = Error;

    fn try_from(document: Document) -> Result<Element, Self::Error> {
        Element::try_from(&document)
    }
}

impl TryFrom<web_sys::Event> for Element {
    type Error = Error;

    fn try_from(event: web_sys::Event) -> Result<Element, Self::Error> {
        if let Some(target) = event.target() {
            Ok(Element::from(
                target
                    .dyn_into::<web_sys::Element>()
                    .map_err(|_| Error::NoTargetElement)?,
            ))
        } else {
            Err(Error::NoTargetElement)
        }
    }
}

/// HTML `Collection` that can be used as an iterator
#[derive(AsRef, Clone, Debug, Default, Deref, DerefMut, From, Into)]
pub struct Collection(pub VecDeque<Element>);

impl Collection {
    pub fn new() -> Self {
        Default::default()
    }

    /// Move all elements of another collection into this collection.
    pub fn append_collection(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    pub fn descendants(&self) -> Collection {
        let mut all_children = Collection::new();
        self.0
            .iter()
            .for_each(|elem| all_children.append_collection(elem.descendants()));
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

impl From<NodeList> for Collection {
    fn from(list: NodeList) -> Self {
        let mut inner = VecDeque::new();

        for i in 0..list.length() {
            if let Some(item) = list.item(i) {
                // Only add `Element` nodes, ignore the others.
                if let Ok(element) = item.dyn_into::<web_sys::Element>() {
                    inner.push_back(element.into());
                }
            }
        }

        Self(inner)
    }
}

impl From<Element> for Collection {
    fn from(element: Element) -> Self {
        Self(vec![element].into())
    }
}

impl FromIterator<Collection> for Collection {
    fn from_iter<I: IntoIterator<Item = Collection>>(iter: I) -> Self {
        iter.into_iter().map(|coll| coll.0).flatten().collect()
    }
}

impl FromIterator<VecDeque<Element>> for Collection {
    fn from_iter<I: IntoIterator<Item = VecDeque<Element>>>(iter: I) -> Self {
        iter.into_iter().flatten().collect()
    }
}

impl FromIterator<Element> for Collection {
    fn from_iter<I: IntoIterator<Item = Element>>(iter: I) -> Self {
        VecDeque::from_iter(iter).into()
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
