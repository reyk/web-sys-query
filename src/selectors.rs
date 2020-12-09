//! CSS selectors for the query.
//!
//! With some inspiration from:
//! Kuchiki (朽木), a HTML/XML tree manipulation library for Rust.
//! https://github.com/kuchiki-rs/kuchiki/blob/master/src/select.rs

use crate::{query::Element, Error};
use cssparser::{CowRcStr, ParseError, SourceLocation, ToCss};
use selectors::{
    self,
    attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
    context::QuirksMode,
    matching::{self, ElementSelectorFlags, MatchingContext},
    parser::{self, NonTSPseudoClass, SelectorParseErrorKind},
    OpaqueElement, SelectorImpl, SelectorList,
};
use std::{borrow::Borrow, fmt, ops::Deref, str::FromStr};

#[derive(Debug, Clone)]
pub struct SelectorTypes;

impl parser::SelectorImpl for SelectorTypes {
    type ExtraMatchingData = ();
    type AttrValue = String;
    type Identifier = String;
    type ClassName = String;
    type PartName = String;
    type LocalName = String;
    type NamespaceUrl = String;
    type NamespacePrefix = String;
    type BorrowedNamespaceUrl = String;
    type BorrowedLocalName = String;
    type NonTSPseudoClass = PseudoClass;
    type PseudoElement = PseudoElement;
}

struct Parser;

impl<'i> parser::Parser<'i> for Parser {
    type Impl = SelectorTypes;
    type Error = parser::SelectorParseErrorKind<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<PseudoClass, ParseError<'i, SelectorParseErrorKind<'i>>> {
        if name.eq_ignore_ascii_case("active") {
            Ok(PseudoClass::Active)
        } else if name.eq_ignore_ascii_case("any-link") {
            Ok(PseudoClass::AnyLink)
        } else if name.eq_ignore_ascii_case("checked") {
            Ok(PseudoClass::Checked)
        } else if name.eq_ignore_ascii_case("disabled") {
            Ok(PseudoClass::Disabled)
        } else if name.eq_ignore_ascii_case("enabled") {
            Ok(PseudoClass::Enabled)
        } else if name.eq_ignore_ascii_case("focus") {
            Ok(PseudoClass::Focus)
        } else if name.eq_ignore_ascii_case("hover") {
            Ok(PseudoClass::Hover)
        } else if name.eq_ignore_ascii_case("indeterminate") {
            Ok(PseudoClass::Indeterminate)
        } else if name.eq_ignore_ascii_case("link") {
            Ok(PseudoClass::Link)
        } else if name.eq_ignore_ascii_case("visited") {
            Ok(PseudoClass::Visited)
        } else {
            Err(
                location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
                    name,
                )),
            )
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum PseudoClass {
    Active,
    AnyLink,
    Checked,
    Disabled,
    Enabled,
    Focus,
    Hover,
    Indeterminate,
    Link,
    Visited,
}

impl NonTSPseudoClass for PseudoClass {
    type Impl = SelectorTypes;

    fn is_active_or_hover(&self) -> bool {
        matches!(*self, PseudoClass::Active | PseudoClass::Hover)
    }

    fn is_user_action_state(&self) -> bool {
        matches!(
            *self,
            PseudoClass::Active | PseudoClass::Hover | PseudoClass::Focus
        )
    }

    fn has_zero_specificity(&self) -> bool {
        false
    }
}

impl ToCss for PseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(match *self {
            PseudoClass::Active => ":active",
            PseudoClass::AnyLink => ":any-link",
            PseudoClass::Checked => ":checked",
            PseudoClass::Disabled => ":disabled",
            PseudoClass::Enabled => ":enabled",
            PseudoClass::Focus => ":focus",
            PseudoClass::Hover => ":hover",
            PseudoClass::Indeterminate => ":indeterminate",
            PseudoClass::Link => ":link",
            PseudoClass::Visited => ":visited",
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum PseudoElement {}

impl ToCss for PseudoElement {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match *self {}
    }
}

impl parser::PseudoElement for PseudoElement {
    type Impl = SelectorTypes;
}

impl selectors::Element for Element {
    type Impl = SelectorTypes;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self)
    }

    fn parent_element(&self) -> Option<Self> {
        self.parent()
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev(None)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next(None)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName) -> bool {
        self.local_name().deref() == local_name
    }

    fn has_namespace(&self, ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool {
        self.namespace_uri().as_ref() == Some(ns)
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.local_name() == other.local_name() && self.namespace_uri() == other.namespace_uri()
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>,
    ) -> bool {
        match *ns {
            NamespaceConstraint::Any => self.get_attribute(local_name),
            NamespaceConstraint::Specific(ns) => self.get_attribute_ns(Some(ns), local_name),
        }
        .filter(|value| operation.eval_str(value))
        .is_some()
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _context: &mut MatchingContext<'_, Self::Impl>,
        _flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, ElementSelectorFlags),
    {
        match *pc {
            PseudoClass::AnyLink | PseudoClass::Link => {
                self.is_link() && self.get_attribute("href").is_some()
            }
            _ => false,
        }
    }

    fn match_pseudo_element(
        &self,
        pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        _context: &mut MatchingContext<'_, Self::Impl>,
    ) -> bool {
        match *pe {}
    }

    fn is_link(&self) -> bool {
        let local_name = self.local_name().to_lowercase();
        matches!(local_name.as_ref(), "a" | "area" | "link")
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn assigned_slot(&self) -> Option<Self> {
        None
    }

    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        case_sensitivity.eq(id.as_bytes(), self.id().as_bytes())
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::ClassName,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        let classes = self.class_list();
        for i in 0..classes.length() {
            if let Some(class) = classes.get(i) {
                if case_sensitivity.eq(name.as_bytes(), class.as_bytes()) {
                    return true;
                }
            }
        }

        false
    }

    fn exported_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::PartName,
    ) -> Option<<Self::Impl as SelectorImpl>::PartName> {
        None
    }

    fn imported_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::PartName,
    ) -> Option<<Self::Impl as SelectorImpl>::PartName> {
        None
    }

    fn is_part(&self, _name: &<Self::Impl as SelectorImpl>::PartName) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        !(self.child_element_count() > 0 && self.text_content().is_some())
    }

    fn is_root(&self) -> bool {
        match self.parent_node() {
            Some(parent) => {
                parent.node_type() == web_sys::Node::DOCUMENT_NODE
                    || parent.node_type() == web_sys::Node::DOCUMENT_FRAGMENT_NODE
            }
            None => false,
        }
    }
}

pub struct Selector(parser::Selector<SelectorTypes>);

impl Selector {
    pub fn matches(&self, element: &Element) -> bool {
        let mut context = matching::MatchingContext::new(
            matching::MatchingMode::Normal,
            None,
            None,
            QuirksMode::NoQuirks,
        );

        matching::matches_selector(&self.0, 0, None, element, &mut context, &mut |_, _| {})
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_css(f)
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn new(selectors: &str) -> Result<Self, Error> {
        let mut input = cssparser::ParserInput::new(selectors);
        SelectorList::parse(&Parser, &mut cssparser::Parser::new(&mut input))
            .map(|list| Selectors(list.0.into_iter().map(Selector).collect()))
            .map_err(|_err| Error::SelectorsParserError)
    }

    pub fn matches(&self, element: &Element) -> bool {
        self.0.iter().any(|s| s.matches(element))
    }

    pub fn filter<I>(&self, iter: I) -> Select<I, &Selectors>
    where
        I: Iterator<Item = Element>,
    {
        Select {
            iter,
            selectors: self,
        }
    }
}

impl FromStr for Selectors {
    type Err = Error;

    fn from_str(selectors: &str) -> Result<Self, Self::Err> {
        Self::new(selectors)
    }
}

impl fmt::Display for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, selector) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            selector.0.to_css(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct Select<I, S = Selectors>
where
    I: Iterator<Item = Element>,
    S: Borrow<Selectors>,
{
    pub iter: I,
    pub selectors: S,
}

impl<I, S> Iterator for Select<I, S>
where
    I: Iterator<Item = Element>,
    S: Borrow<Selectors>,
{
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let selectors = self.selectors.borrow();
        self.iter.by_ref().find(|elem| selectors.matches(elem))
    }
}

impl<I, S> DoubleEndedIterator for Select<I, S>
where
    I: DoubleEndedIterator<Item = Element>,
    S: Borrow<Selectors>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let selectors = self.selectors.borrow();
        self.iter
            .by_ref()
            .rev()
            .find(|elem| selectors.matches(elem))
    }
}
