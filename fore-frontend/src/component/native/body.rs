//! The HTML `body` element.

use crate::{AttributeMap, Children, Component, DOMElement, HTMLElement};

/// The HTML `body` element as a component.
#[derive(Clone, Debug, PartialEq)]
pub struct Body {}

impl Component for Body {
    fn new() -> Self {
        Self {}
    }

    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement {
        let mut element = HTMLElement::new("body".into());
        *element.get_attributes_mut() = attrs;
        *element.get_children_mut() = children;
        element
    }
}