//! The frontend elements of Fore.

pub mod component;
pub mod element;
pub mod global_attr;

pub use component::Component;
pub use element::{AttributeMap, Children, DOMElement, ElementGroup, HTMLElement, HTMLElementBuilder,
                  TextNode};
pub use global_attr::global_attr;
pub use std::fmt::{Debug, Display};


impl From<&str> for Box<dyn DOMElement> {
    fn from(src: &str) -> Self {
        Box::new(TextNode::new(src.to_string()))
    }
}

impl From<String> for Box<dyn DOMElement> {
    fn from(src: String) -> Self {
        Box::new(TextNode::new(src))
    }
}

impl From<TextNode> for Box<dyn DOMElement> {
    fn from(src: TextNode) -> Self {
        Box::new(src)
    }
}

impl From<HTMLElement> for Box<dyn DOMElement> {
    fn from(src: HTMLElement) -> Self {
        Box::new(src)
    }
}

impl From<ElementGroup> for Box<dyn DOMElement> {
    fn from(src: ElementGroup) -> Self {
        Box::new(src)
    }
}

impl From<bool> for Box<dyn DOMElement> {
    fn from(src: bool) -> Self {
        Box::new(TextNode::new(src.to_string()))
    }
}

impl From<&dyn ToString> for Box<dyn DOMElement> {
    fn from(src: &dyn ToString) -> Self {
        Box::new(TextNode::new(src.to_string()))
    }
}

impl From<&dyn Display> for Box<dyn DOMElement> {
    fn from(src: &dyn Display) -> Self {
        Box::new(TextNode::new(format!("{}", src)))
    }
}

impl From<&dyn Debug> for Box<dyn DOMElement> {
    fn from(src: &dyn Debug) -> Self {
        Box::new(TextNode::new(format!("{:?}", src)))
    }
}