//! Elements for the Fore DOM.

use std::collections::HashMap;

/// A list of attributes of a DOM element.
pub type AttributeMap = HashMap<String, String>;

/// A list of children in a DOM element.
pub type Children = Vec<Box<dyn DOMElement>>;

/// An element residing in the Fore DOM.
pub trait DOMElement {
    /// Gets all attributes of this DOM element.
    fn get_attributes(&self) -> &AttributeMap;

    /// Gets all attributes of this DOM element, in a mutable state.
    fn get_attributes_mut(&mut self) -> &mut AttributeMap;

    /// Gets an attribute of this DOM element by name.
    fn get_attribute(&self, name: String) -> Option<&String> {
        self.get_attributes().get(&name)
    }

    /// Sets an attribute of this DOM element by name.
    fn set_attribute(&mut self, name: String, value: String) {
        self.get_attributes_mut().insert(name, value);
    }

    /// Returns the children in this element.
    fn get_children(&self) -> &Children;

    /// Returns the children in this element, in a mutable state.
    fn get_children_mut(&mut self) -> &mut Children;

    /// Appends a child to this element.
    fn append_child(&mut self, child: Box<dyn DOMElement>) {
        self.get_children_mut().push(child);
    }

    /// Appends a child before every other child of this element.
    fn append_child_first(&mut self, child: Box<dyn DOMElement>) {
        self.get_children_mut().insert(0, child);
    }

    /// Sets the inner text of this DOM element.
    /// 
    /// If this element is an HTML element, it clears all children and inserts a text node.  If this
    /// element is a text node, it simply sets the text of it.
    fn set_inner_text(&mut self, text: String);

    /// Returns the rendered inner text of this element.
    fn get_inner_text(&self) -> String;

    /// Renders this DOM element as a string.
    fn render(&self) -> String;
}

/// An HTML element for the Fore DOM.
pub struct HTMLElement {
    /// The tag name of the HTML element.
    tag_name: String,

    /// The attributes of this element.
    attributes: AttributeMap,

    /// The children of this element.
    children: Children,
}

impl HTMLElement {
    /// Creates a new HTML element.
    pub fn new(tag_name: String) -> Self {
        Self {
            tag_name,
            attributes: HashMap::new(),
            children: vec![],
        }
    }

    /// Returns the tag name of this HTML element.
    pub fn get_tag_name(&self) -> &String {
        &self.tag_name
    }
}

impl DOMElement for HTMLElement {
    fn get_attributes(&self) -> &AttributeMap {
        &self.attributes
    }

    fn get_attributes_mut(&mut self) -> &mut AttributeMap {
        &mut self.attributes
    }

    fn get_children(&self) -> &Children {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Children {
        &mut self.children
    }

    fn set_inner_text(&mut self, value: String) {
        self.children.clear();
        self.children.push(Box::new(TextNode::new(value)));
    }

    fn get_inner_text(&self) -> String {
        let mut inner_text = String::new();

        for child in &self.children {
            inner_text.push_str(&child.render());
        }

        inner_text
    }

    fn render(&self) -> String {
        let mut rendered = String::new();
        rendered.push_str(&format!("<{}", self.tag_name));

        for (name, value) in &self.attributes {
            rendered.push(' ');

            if value == "true" {
                rendered.push_str(name);
            } else {
                rendered.push_str(&format!("{}=\"{}\"", name, value));
            }
        }
        
        rendered.push_str(&format!(">{}</{}>", self.get_inner_text(), self.tag_name));

        rendered
    }
}

/// An HTML text node for the Fore DOM.
pub struct TextNode {
    /// The text in the node.
    value: String,
}

impl TextNode {
    /// Creates a new text node.
    pub fn new(value: String) -> Self {
        Self {
            value,
        }
    }
}

impl DOMElement for TextNode {
    fn get_attributes(&self) -> &AttributeMap {
        panic!("Text nodes don't have attributes.");
    }

    fn get_attributes_mut(&mut self) -> &mut AttributeMap {
        panic!("Text nodes don't have attributes.");
    }

    fn get_children(&self) -> &Children {
        panic!("Text nodes don't have children.");
    }

    fn get_children_mut(&mut self) -> &mut Children {
        panic!("Text nodes don't have children.");
    }

    fn set_inner_text(&mut self, value: String) {
        self.value = value;
    }

    fn get_inner_text(&self) -> String {
        self.value.clone()
    }

    fn render(&self) -> String {
        self.value.clone()
    }
}

/// An easy-to-use element builder with nice looking chaining.
pub struct HTMLElementBuilder {
    /// The element that is being built.
    element: HTMLElement,
}

impl HTMLElementBuilder {
    /// Creates a new HTML element builder.
    pub fn new<Str: Into<String>>(tag_name: Str) -> Self {
        Self {
            element: HTMLElement::new(tag_name.into())
        }
    }

    /// Adds an attribute to the HTML element.
    pub fn with_attr<Str: Into<String>>(mut self, name: Str, value: Str) -> Self {
        self.element.set_attribute(name.into(), value.into());
        self
    }
    
    /// Adds an attribute to the HTML element.
    pub fn with_child(mut self, child: Box<dyn DOMElement>) -> Self {
        self.element.append_child(child);
        self
    }

    /// Sets the text of the HTML element.
    pub fn with_text<Str: Into<String>>(mut self, text: Str) -> Self {
        self.element.set_inner_text(text.into());
        self
    }

    /// Finishes building the HTML element and returns it.
    pub fn finish(self) -> HTMLElement {
        self.element
    }
}

/// A group of HTML elements.
pub struct ElementGroup {
    /// The children of this group.
    children: Children,
}

impl ElementGroup {
    /// Creates a new group.
    pub fn new() -> Self {
        Self {
            children: vec![],
        }
    }
}

impl DOMElement for ElementGroup {
    fn get_attributes(&self) -> &AttributeMap {
        panic!("Text nodes don't have attributes.");
    }

    fn get_attributes_mut(&mut self) -> &mut AttributeMap {
        panic!("Text nodes don't have attributes.");
    }

    fn get_children(&self) -> &Children {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Children {
        &mut self.children
    }

    fn set_inner_text(&mut self, value: String) {
        self.children.clear();
        self.children.push(Box::new(TextNode::new(value)));
    }

    fn get_inner_text(&self) -> String {
        let mut inner_text = String::new();

        for child in &self.children {
            inner_text.push_str(&child.render());
        }

        inner_text
    }

    fn render(&self) -> String {
        self.get_inner_text()
    }
}