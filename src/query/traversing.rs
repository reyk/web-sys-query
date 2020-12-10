//! Traversing

use super::*;

impl Element {
    // TODO: .add()
    // TODO: .addBack()

    pub fn children(&self, selectors: Option<&Selectors>) -> Collection {
        if let Some(selectors) = selectors {
            selectors
                .filter(Collection::from(self.0.children()).into_iter())
                .collect::<Vec<_>>()
                .into()
        } else {
            self.0.children().into()
        }
    }

    // TODO: .closest()
    // TODO: .contents()
    // TODO: .each()
    // TODO: .end()
    // TODO: .eq()
    // TODO: .even()

    /// Filter if the element or its decendants matches the selector.
    ///
    /// NOTE: `filter([function])` is not implemented and can be done
    /// with Rust iterators instead.
    pub fn filter(&self, selectors: &Selectors) -> Option<Self> {
        if selectors.matches(self) {
            Some(self.clone())
        } else {
            self.has(selectors)
        }
    }

    /// Find elements by selectors.
    pub fn find(&self, selectors: &Selectors) -> Collection {
        selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn first(&mut self) -> Element {
        self.clone()
    }

    /// Filter if a decendant matches the selector.
    pub fn has(&self, selectors: &Selectors) -> Option<Self> {
        if self
            .descendants()
            .iter()
            .any(|elem| selectors.matches(elem))
        {
            Some(self.clone())
        } else {
            None
        }
    }

    /// Check if the element matches the selectors.
    pub fn is(&self, selectors: &Selectors) -> bool {
        selectors.matches(self)
            || self
                .descendants()
                .iter()
                .any(|elem| selectors.matches(elem))
    }

    pub fn last(&mut self) -> Element {
        self.clone()
    }

    // TODO: .map()

    pub fn next(&self, selectors: Option<&Selectors>) -> Option<Self> {
        if let Some(element) = self.next_element_sibling().map(Into::into) {
            if matches!(selectors, Some(selectors) if !selectors.matches(&element)) {
                return None;
            }
            Some(element)
        } else {
            None
        }
    }

    // TODO: .nextAll()
    // TODO: .nextUntil()
    // TODO: .not()
    // TODO: .odd()
    // TODO: .offsetParent()

    pub fn parent(&self) -> Option<Self> {
        self.parent_element().map(Into::into)
    }

    // TODO: .parents()
    // TODO: .parentsUntil()

    pub fn prev(&self, selectors: Option<&Selectors>) -> Option<Self> {
        if let Some(element) = self.previous_element_sibling().map(Into::into) {
            if matches!(selectors, Some(selectors) if !selectors.matches(&element)) {
                return None;
            }
            Some(element)
        } else {
            None
        }
    }

    // TODO: .prevAll()
    // TODO: .prevUntil()
    // TODO: .siblings()
    // TODO: .slice()
}

impl Collection {
    pub fn children(&self, selectors: Option<&Selectors>) -> Collection {
        let mut all_children = Collection::new();
        for element in self.0.iter() {
            all_children.append_collection(element.children(selectors));
        }
        all_children
    }

    pub fn filter(&self, selectors: &Selectors) -> Collection {
        self.0
            .iter()
            .filter_map(|elem| elem.filter(selectors))
            .collect::<Vec<_>>()
            .into()
    }

    pub fn find(&self, selectors: &Selectors) -> Collection {
        selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn first(&mut self) -> Result<Element, Error> {
        self.0
            .front()
            .ok_or(Error::FirstElementNotFound)
            .map(ToOwned::to_owned)
    }

    pub fn has(&self, selectors: &Selectors) -> Collection {
        self.0
            .iter()
            .filter_map(|elem| elem.has(selectors))
            .collect::<Vec<_>>()
            .into()
    }

    pub fn is(&self, selectors: &Selectors) -> bool {
        self.0.iter().any(|elem| elem.is(selectors))
    }

    pub fn last(&mut self) -> Result<Element, Error> {
        self.0
            .back()
            .ok_or(Error::FirstElementNotFound)
            .map(ToOwned::to_owned)
    }

    pub fn next(&self, selectors: Option<&Selectors>) -> Collection {
        self.0
            .iter()
            .filter_map(|elem| elem.next(selectors))
            .collect::<Vec<_>>()
            .into()
    }

    pub fn parent(&self) -> Collection {
        self.0
            .iter()
            .filter_map(|elem| elem.parent())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn prev(&self, selectors: Option<&Selectors>) -> Collection {
        self.0
            .iter()
            .filter_map(|elem| elem.prev(selectors))
            .collect::<Vec<_>>()
            .into()
    }
}

macro_rules! document {
    ($self:ident, $cb:expr) => {
        Element::try_from($self).map($cb).unwrap_or_default()
    };
}

impl Document {
    pub fn children(&self, selectors: Option<&Selectors>) -> Collection {
        document!(self, |elem| elem.children(selectors))
    }

    pub fn find(&self, selectors: &Selectors) -> Collection {
        document!(self, |elem| elem.find(selectors))
    }
}
