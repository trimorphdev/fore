//! The HTML `div` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};
use crate::global_attr;

/// The HTML `div` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Div {}

impl Component for Div {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("div".into());
        *element.get_attributes_mut() = global_attr(&attrs);
        *element.get_children_mut() = children;
        element
    }
}