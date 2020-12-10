//! Attributes

use super::*;

impl Element {
    /// Add CSS class.
    pub fn add_class(&self, class: &str) -> Result<(), Error> {
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

    /// Check if element has a matching CSS class.
    pub fn has_class(&self, class: &str) -> bool {
        self.0.class_list().contains(class)
    }

    /// Get the inner HTML of the element.
    pub fn html(&self) -> String {
        self.0.inner_html()
    }

    /// Set the inner HTML of the element.
    pub fn set_html(&self, html: &str) {
        self.0.set_inner_html(html)
    }

    // TODO: .prop()

    /// Remove the attribute.
    pub fn remove_attr(&self, key: &str) -> Result<(), Error> {
        if key == "id" {
            return Err(Error::CannotRemoveAttribute(self.0.id()));
        }

        self.0.remove_attribute(key).map_err(Into::into)
    }

    /// Remove CSS class from list.
    pub fn remove_class(&self, class: &str) -> Result<(), Error> {
        self.0.class_list().remove_1(class).map_err(Into::into)
    }

    // TODO: .remove_prop()

    /// Return the matching attribute, if found.
    pub fn toggle_class(&self, class: &str) -> Result<bool, Error> {
        self.0.class_list().toggle(class).map_err(Into::into)
    }

    pub fn val(&self) -> Result<String, Error> {
        // TODO: there must be a nicer and generic way to figure out
        // if the Node has a `value` property.  jQuery looks for the
        // value property or function but I'm not sure if this can be
        // done in Rust on the raw `JsValue` object.
        if let Some(node) = self.0.dyn_ref::<web_sys::HtmlButtonElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlDataElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlInputElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlOptionElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlOutputElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlParamElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlSelectElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlTextAreaElement>() {
            Ok(node.value())
        } else {
            Err(Error::NoValue("string"))
        }
    }

    pub fn val_f64(&self) -> Result<f64, Error> {
        if let Some(node) = self.0.dyn_ref::<web_sys::HtmlMeterElement>() {
            Ok(node.value())
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlProgressElement>() {
            Ok(node.value())
        } else {
            Err(Error::NoValue("float"))
        }
    }

    pub fn val_i32(&self) -> Result<i32, Error> {
        if let Some(node) = self.0.dyn_ref::<web_sys::HtmlLiElement>() {
            Ok(node.value())
        } else {
            Err(Error::NoValue("float"))
        }
    }

    pub fn set_val(&self, value: &str) -> Result<(), Error> {
        if let Some(node) = self.0.dyn_ref::<web_sys::HtmlButtonElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlDataElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlInputElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlOptionElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlOutputElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlParamElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlSelectElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlTextAreaElement>() {
            node.set_value(value)
        } else {
            return Err(Error::NoValue("string"));
        }

        Ok(())
    }

    pub fn set_val_f64(&self, value: f64) -> Result<(), Error> {
        if let Some(node) = self.0.dyn_ref::<web_sys::HtmlMeterElement>() {
            node.set_value(value)
        } else if let Some(node) = self.0.dyn_ref::<web_sys::HtmlProgressElement>() {
            node.set_value(value)
        } else {
            return Err(Error::NoValue("float"));
        }

        Ok(())
    }

    pub fn set_val_i32(&self, value: i32) -> Result<(), Error> {
        if let Some(node) = self.0.dyn_ref::<web_sys::HtmlLiElement>() {
            node.set_value(value)
        } else {
            return Err(Error::NoValue("float"));
        }

        Ok(())
    }
}

impl Collection {
    pub fn add_class(&self, class: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.add_class(class)?;
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

    pub fn has_class(&self, class: &str) -> bool {
        self.0.iter().any(|elem| elem.has_class(class))
    }

    pub fn html(&self) -> String {
        self.0.iter().map(|elem| elem.html()).collect()
    }

    pub fn set_html(&self, html: &str) {
        self.0.iter().for_each(|elem| elem.set_html(html))
    }

    pub fn remove_attr(&self, key: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.remove_attr(key)?;
        }

        Ok(())
    }

    pub fn remove_class(&self, class: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.remove_class(class)?;
        }

        Ok(())
    }

    /// Return the matching attribute, if found.
    pub fn toggle_class(&self, class: &str) -> Result<(), Error> {
        for element in self.0.iter() {
            element.toggle_class(class)?;
        }

        Ok(())
    }

    pub fn val(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|elem| elem.val().unwrap_or_default())
            .collect()
    }

    pub fn val_f64(&self) -> Vec<f64> {
        self.0
            .iter()
            .map(|elem| elem.val_f64().unwrap_or_default())
            .collect()
    }

    pub fn val_i32(&self) -> Vec<i32> {
        self.0
            .iter()
            .map(|elem| elem.val_i32().unwrap_or_default())
            .collect()
    }

    pub fn set_val(&self, value: &str) {
        self.0.iter().for_each(|elem| {
            elem.set_val(value).ok();
        });
    }

    pub fn set_val_f64(&self, value: f64) {
        self.0.iter().for_each(|elem| {
            elem.set_val_f64(value).ok();
        });
    }

    pub fn set_val_i32(&self, value: i32) {
        self.0.iter().for_each(|elem| {
            elem.set_val_i32(value).ok();
        });
    }
}
