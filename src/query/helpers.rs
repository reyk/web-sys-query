//! Helper Functions

use crate::{
    error::Error,
    query::{Collection, Element},
};
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use std::iter::FromIterator;
use web_sys::HtmlFormElement;

impl Element {
    /// Serialize a form into a data structure that holds a key-value
    /// type.
    ///
    /// The generic return type can be anything that supports the
    /// trait bounds, for example `Vec<(String, String)>` or
    /// `HashMap<String, String>`.  To get a jQuery-compatible return
    /// type, use the `FormData` type that is provided by this crate.
    pub fn serialize_array<T>(&self) -> Result<T, Error>
    where
        T: FromIterator<(String, String)>,
    {
        let form = self.dyn_ref::<HtmlFormElement>()?;

        Collection::from(form.elements())
            .into_iter()
            .filter(|elem| !elem.is(":disabled").unwrap_or_default())
            .map(|elem| {
                let key = elem.attr("name").ok_or(Error::NoDocumentElement)?;
                let value = elem.val()?;
                Ok((key, value))
            })
            .collect::<Result<T, Error>>()
    }
}

impl Collection {
    /// Serialize a collection of forms into a data structure that
    /// holds a key-value type.
    ///
    /// The generic return type can be anything that supports the
    /// trait bounds, for example `Vec<(String, String)>` or
    /// `HashMap<String, String>`.  To get a jQuery-compatible return
    /// type, use the `FormData` type that is provided by this crate.
    pub fn serialize_array<T>(&self) -> Result<T, Error>
    where
        T: FromIterator<(String, String)> + IntoIterator + FromIterator<<T as IntoIterator>::Item>,
    {
        Ok(self
            .0
            .iter()
            .map(|elem| elem.serialize_array::<T>())
            .collect::<Result<Vec<T>, Error>>()?
            .into_iter()
            .flatten()
            .collect::<T>())
    }
}

/// "name-value" representation of form data.
///
/// This struct can be used with the `.serialize_array()` function to
/// get jquery-compatible name-value representation of the result.
/// Some form elements, such as radio buttons or checkboxes, can
/// contain multiple elements with the same name, so this array of
/// name-value fields is returned instead of simple `HashMap`.
#[derive(AsRef, Debug, Deref, DerefMut, Eq, From, PartialEq, Into)]
#[cfg_attr(
    feature = "serde-serialize",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct FormData(Vec<FormValue>);

impl FromIterator<(String, String)> for FormData {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl FromIterator<FormValue> for FormData {
    fn from_iter<I: IntoIterator<Item = FormValue>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl IntoIterator for FormData {
    type Item = FormValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// "name-value" representation of a single `FormData` field.
#[derive(Debug, Eq, From, Into, PartialEq)]
#[cfg_attr(
    feature = "serde-serialize",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct FormValue {
    pub name: String,
    pub value: String,
}
