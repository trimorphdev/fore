//! The HTML `p` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};

/// The HTML `p` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct P {}

impl Component for P {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("p".into());
        *element.get_attributes_mut() = attrs;
        *element.get_children_mut() = children;
        element
    }
}