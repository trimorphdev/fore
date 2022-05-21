//! The HTML `span` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};
use crate::global_attr;

/// The HTML `span` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {}

impl Component for Span {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("span".into());
        *element.get_attributes_mut() = global_attrs(&attrs);
        *element.get_children_mut() = children;
        element
    }
}