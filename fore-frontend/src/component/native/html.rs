//! The HTML `html` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};
use crate::global_attr;

/// The HTML `html` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Html {}

impl Component for Html {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("html".into());
        *element.get_attributes_mut() = global_attr(&attrs);
        *element.get_children_mut() = children;
        element
    }
}