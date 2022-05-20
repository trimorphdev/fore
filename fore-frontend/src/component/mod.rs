//! Components for the Fore framework.

pub mod native;

use crate::{AttributeMap, Children, HTMLElement};
use std::fmt::Debug;

/// A component which can be rendered into an HTML element.
pub trait Component: Debug + PartialEq {
    /// Creates a new instance of this component.
    fn new() -> Self;

    /// Renders the component into an HTML element.
    fn render(&self, attrs: AttributeMap, children: Children) -> HTMLElement;
}