//! Traversing

use crate::{
    error::Error,
    query::{Collection, Document, Element},
};

/// Traversing methods
impl Element {
    // TODO: .add()
    // TODO: .addBack()

    pub fn children(&self, selectors: Option<&str>) -> Result<Collection, Error> {
        let children = Collection::from(self.0.children());
        if let Some(selectors) = selectors {
            children
                .iter()
                .filter_map(|elem| match elem.is(selectors) {
                    Err(err) => Some(Err(err)),
                    Ok(true) => Some(Ok(elem.clone())),
                    Ok(false) => None,
                })
                .collect::<Result<Vec<Element>, Error>>()
                .map(Into::into)
        } else {
            Ok(children)
        }
    }

    /// Find the closets element that matches the selector.
    pub fn closest(&self, selectors: &str) -> Result<Option<Self>, Error> {
        self.0
            .closest(selectors)
            .map(|elem| elem.map(Into::into))
            .map_err(Into::into)
    }

    // TODO: .contents()
    // TODO: .each()
    // TODO: .end()
    // TODO: .eq()
    // TODO: .even()

    /// Filter if the element matches the selector.
    pub fn filter(&self, selectors: &str) -> Result<Option<Self>, Error> {
        self.is(selectors)
            .map(|result| if result { Some(self.clone()) } else { None })
    }

    /// Find elements by selectors.
    pub fn find(&self, selectors: &str) -> Result<Collection, Error> {
        self.0
            .query_selector_all(selectors)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn first(&self) -> Element {
        self.clone()
    }

    /// Filter if a decendant matches the selector.
    pub fn has(&self, selectors: &str) -> Result<Option<Self>, Error> {
        self.0
            .query_selector(selectors)
            .map(|elem| elem.map(|_| self.clone()))
            .map_err(Into::into)
    }

    /// Check if the element matches the selectors.
    pub fn is(&self, selectors: &str) -> Result<bool, Error> {
        self.0.matches(selectors).map_err(Into::into)
    }

    pub fn last(&self) -> Element {
        self.clone()
    }

    // TODO: .map()

    pub fn next(&self, selectors: Option<&str>) -> Result<Option<Self>, Error> {
        if let Some(element) = self.next_element_sibling().map(Self::from) {
            match selectors {
                Some(selectors) => element.filter(selectors),
                None => Ok(Some(element)),
            }
        } else {
            Ok(None)
        }
    }

    // TODO: .nextAll()
    // TODO: .nextUntil()

    /// Filter if the element does not matche the selector.
    pub fn not(&self, selectors: &str) -> Result<Option<Self>, Error> {
        self.is(selectors)
            .map(|result| if !result { Some(self.clone()) } else { None })
    }

    // TODO: .odd()
    // TODO: .offsetParent()

    pub fn parent(&self) -> Option<Self> {
        self.parent_element().map(Into::into)
    }

    // TODO: .parents()
    // TODO: .parentsUntil()

    pub fn prev(&self, selectors: Option<&str>) -> Result<Option<Self>, Error> {
        if let Some(element) = self.previous_element_sibling().map(Self::from) {
            match selectors {
                Some(selectors) => element.filter(selectors),
                None => Ok(Some(element)),
            }
        } else {
            Ok(None)
        }
    }

    // TODO: .prevAll()
    // TODO: .prevUntil()
    // TODO: .siblings()
    // TODO: .slice()
}

/// Traversing methods
impl Collection {
    pub fn children(&self, selectors: Option<&str>) -> Result<Collection, Error> {
        self.iter().map(|elem| elem.children(selectors)).collect()
    }

    pub fn filter(&self, selectors: &str) -> Result<Collection, Error> {
        self.iter()
            .filter_map(|elem| elem.filter(selectors).transpose())
            .collect()
    }

    pub fn find(&self, selectors: &str) -> Result<Collection, Error> {
        self.iter().map(|elem| elem.find(selectors)).collect()
    }

    pub fn first(&self) -> Option<Element> {
        self.0.front().map(Clone::clone)
    }

    pub fn has(&self, selectors: &str) -> Result<Collection, Error> {
        self.iter()
            .filter_map(|elem| elem.has(selectors).transpose())
            .collect()
    }

    pub fn is(&self, selectors: &str) -> Result<bool, Error> {
        let is = self
            .iter()
            .map(|elem| elem.is(selectors))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(is.contains(&true))
    }

    pub fn last(&self) -> Option<Element> {
        self.0.back().map(Clone::clone)
    }

    pub fn next(&self, selectors: Option<&str>) -> Result<Collection, Error> {
        self.iter()
            .filter_map(|elem| elem.next(selectors).transpose())
            .collect()
    }

    pub fn parent(&self) -> Collection {
        self.iter().filter_map(|elem| elem.parent()).collect()
    }

    pub fn prev(&self, selectors: Option<&str>) -> Result<Collection, Error> {
        self.iter()
            .filter_map(|elem| elem.prev(selectors).transpose())
            .collect()
    }
}

/// Traversing methods.
impl Document {
    pub fn children(&self, selectors: Option<&str>) -> Result<Collection, Error> {
        let children = Collection::from(self.0.children());
        if let Some(selectors) = selectors {
            children
                .iter()
                .filter_map(|elem| match elem.is(selectors) {
                    Err(err) => Some(Err(err)),
                    Ok(true) => Some(Ok(elem.clone())),
                    Ok(false) => None,
                })
                .collect::<Result<Vec<Element>, Error>>()
                .map(Into::into)
        } else {
            Ok(children)
        }
    }

    pub fn find(&self, selectors: &str) -> Result<Collection, Error> {
        self.0
            .query_selector_all(selectors)
            .map(Into::into)
            .map_err(Into::into)
    }
}
