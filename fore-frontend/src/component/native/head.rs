//! The HTML `head` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};
use crate::global_attr;

/// The HTML `head` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Head {}

impl Component for Head {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("head".into());
        *element.get_attributes_mut() = global_attr(&attrs);
        *element.get_children_mut() = children;
        element
    }
}