//! The HTML `head` element.
use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};

/// The HTML `head` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Head {}

impl Component for Head {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("head".into());
        *element.get_attributes_mut() = attrs;
        *element.get_children_mut() = children;
        element
    }
}