//! The HTML `title` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};

/// The HTML `title` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Title {}

impl Component for Title {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("title".into());
        *element.get_attributes_mut() = attrs;
        *element.get_children_mut() = children;
        element
    }
}