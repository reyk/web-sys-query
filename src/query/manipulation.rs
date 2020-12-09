//! Manipulation of the DOM

use super::*;

impl Element {
    /// Get the inner text.
    pub fn text(&self) -> Result<String, Error> {
        Ok(self.dyn_ref::<web_sys::HtmlElement>()?.inner_text())
    }

    /// Set the inner text.
    pub fn set_text(&self, text: &str) -> Result<(), Error> {
        self.dyn_ref::<web_sys::HtmlElement>()?.set_inner_text(text);
        Ok(())
    }
}

impl Collection {
    pub fn text(&self) -> Vec<String> {
        self.0.iter().filter_map(|elem| elem.text().ok()).collect()
    }

    pub fn set_text(&self, text: &str) {
        self.0.iter().for_each(|elem| {
            elem.set_text(text).ok();
        })
    }
}
