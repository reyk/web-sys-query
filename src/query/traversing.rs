//! Traversing

use super::*;

impl Element {
    // TODO: .add()
    // TODO: .addBack()

    pub fn children(&self, selectors: Option<&Selectors>) -> Result<Collection, Error> {
        if let Some(selectors) = selectors {
            Ok(selectors
                .filter(Collection::from(self.0.children()).into_iter())
                .collect::<Vec<_>>()
                .into())
        } else {
            Ok(self.0.children().into())
        }
    }

    // TODO: .closest()
    // TODO: .contents()
    // TODO: .each()
    // TODO: .end()
    // TODO: .eq()
    // TODO: .even()
    // TODO: .filter()

    /// Find elements by selectors.
    pub fn find(&self, selectors: &Selectors) -> Result<Collection, Error> {
        Ok(selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into())
    }

    pub fn first(&mut self) -> Element {
        self.clone()
    }

    // TODO: .has()
    // TODO: .is()

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
    pub fn children(&self, selectors: Option<&Selectors>) -> Result<Collection, Error> {
        let mut all_children = Collection::new();
        for element in self.0.iter() {
            all_children.append_collection(element.children(selectors)?);
        }
        Ok(all_children)
    }

    pub fn find(&self, selectors: &Selectors) -> Result<Collection, Error> {
        Ok(selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into())
    }

    pub fn first(&mut self) -> Result<Element, Error> {
        self.0
            .front()
            .ok_or(Error::FirstElementNotFound)
            .map(ToOwned::to_owned)
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

impl Document {
    pub fn find(&self, selectors: &Selectors) -> Result<Collection, Error> {
        Ok(selectors
            .filter(self.descendants().into_iter())
            .collect::<Vec<_>>()
            .into())
    }

    pub fn children(&self) -> Collection {
        self.0.children().into()
    }
}
