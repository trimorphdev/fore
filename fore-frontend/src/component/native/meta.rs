//! The HTML `meta` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};
use crate::global_attr;

/// The HTML `meta` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Meta {}

impl Component for Meta {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("meta".into());
        *element.get_attributes_mut() = global_attr(&attrs);
        *element.get_children_mut() = children;
        element
    }
}