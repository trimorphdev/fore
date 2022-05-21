//! The HTML `link` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};

/// The HTML `link` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Link {}

impl Component for Link {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("link".into());
        *element.get_attributes_mut() = attrs;
        *element.get_children_mut() = children;
        element
    }
}