//! The `rsx` macro for frontend Fore applications.

// TODO: compile to `format` macros instead, which would have much better performance.

extern crate proc_macro;

use proc_macro2::{LineColumn, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use std::collections::HashMap;
use std::iter::Peekable;

/// The result of parse_child.
enum IsChild {
    End,
    Error(TokenStream),
    Some(TokenStream),
}

/// Returns `true` if `span1` is less than `span2`.
fn span_less_than(span1: LineColumn, span2: LineColumn) -> bool {
    dbg!(span1);
    dbg!(span2);
    if span1.line < span2.line {
        return true;
    } else if span1.line == span2.line && span1.column < (span2.column - 1) {
        return true;
    }

    false
}

/// Parses a text node.
fn parse_text_node(iter: &mut Peekable<proc_macro2::token_stream::IntoIter>) -> String {
    let mut value = String::new();
    let mut last_end = LineColumn {
        line: 0,
        column: 0,
    };
    let mut first = true;

    while let Some(next_token) = iter.peek() {
        let as_string = next_token.to_string();
        println!("TEXT NODE ITERATION: '{}'", as_string);
        println!("ITERATION SPAN: '{:?}'", next_token.span());
        if as_string == "<" || as_string == "{" {
            break;
        }

        if first {
            first = false;
            last_end = next_token.span().end();
        }

        if span_less_than(last_end, next_token.span().end()) {
            value.push(' ');
            value.push_str(&next_token.to_string());
        } else {
            value.push_str(&next_token.to_string());
        }

        last_end = next_token.span().end();
        iter.next();
    }

    println!("VALUE FOUND.");
    return value;
}

/// Parses a child of an HTML element.
fn parse_child(iter: &mut Peekable<proc_macro2::token_stream::IntoIter>) -> IsChild {
    if let Some(next_token) = iter.peek() {
        match next_token {
            TokenTree::Group(group) => {
                println!("PARSING CHILD (group)");
                let value = IsChild::Some(group.to_string().parse().unwrap());
                iter.next();
                value
            },
            TokenTree::Punct(punct) => {
                let as_string = punct.to_string();
                if as_string == "<" {
                    iter.next();
                    if let Some(name) = iter.peek() {
                        println!("CHILD NAME: {}", name);
                        if name.to_string() == "/" {
                            // it's a closing tag
                            iter.next();
                            println!("PARSING CHILDREN (END)");
                            return IsChild::End;
                        }
                    } else {
                        println!("PARSING CHILDREN (ERROR)");
                        return IsChild::Error(quote! { compile_error!("expected a tag name") });
                    }
        
                    println!("PARSING CHILDREN (CHILD)");
                    return IsChild::Some(parse_html_element(iter));
                } else {
                    println!("PARSING CHILDREN (PUNCT TEXT)");
                    return IsChild::Some(
                        format!(
                            "fore_frontend::TextNode::new(\"{}\".into())",
                            parse_text_node(iter)).parse().unwrap());
                }
            },
            _ => {
                println!("PARSING CHILDREN (PUNCT TEXT)");
                return IsChild::Some(
                    format!(
                        "fore_frontend::TextNode::new(\"{}\".into())",
                        parse_text_node(iter)).parse().unwrap());
            },
        }
    } else {
        return IsChild::Error(quote! { compile_error!("expected a child component or closing tag") });
    }
}

/// Parses an HTML element and its children.
fn parse_html_element(iter: &mut Peekable<proc_macro2::token_stream::IntoIter>) -> TokenStream {
    let mut is_group = false;
    let name; // the name of the component
    // Get the name of the component
    if let Some(open) = iter.next() {
        let raw_name = open.to_string();

        if raw_name == ">" {
            // is group
            is_group = true;
            name = ">".into();
            println!("IS GROUP");
        } else {
            match open {
                TokenTree::Ident(ident) => {
                    name = raw_name;
    
                    println!("COMPONENT NAME");
                },
                token => {
                    return quote_spanned! {
                        token.span() =>
                        compile_error!("expected a component name");
                    };
                }
            }
        }
    } else {
        return quote! { compile_error!("expected a component name or '>'") };
    }

    println!("Component name: {}" , name);

    let mut attributes = HashMap::new();
    if !is_group {
        // Look for attributes
        while let Some(attr) = iter.next() {
            if attr.to_string() == ">" {
                println!("ATTRIBUTES OVER");
                break;
            }

            // Attribute name
            let attr_name;
            match attr {
                TokenTree::Ident(ident) => {
                    attr_name = ident.to_string();
                    println!("ATTRIBUTE NAME: {}", attr_name);
                },
                token => {
                    return quote_spanned! {
                        token.span() =>
                        compile_error!("expected an attribute or '>'");
                    };
                },
            }

            if let Some(equals) = iter.peek() {
                if equals.to_string() != "=" {
                    attributes.insert(attr_name.to_string(), "\"true\"".to_string());
                    continue;
                }

                iter.next(); // skip over =
                println!("AFTER EQUALS:  {}", iter.peek().unwrap());
            } else {
                return quote! { compile_error!("expected component to close") };
            }

            // get value of attribute.
            let mut attr_value;
            if let Some(value) = iter.next() {
                match value {
                    TokenTree::Literal(literal) => {
                        attr_value = literal.to_string();
                        if &attr_value[..1] == "\"" {
                            attr_value = (&attr_value[1..attr_value.len() - 1]).to_string()
                        }
                        attr_value = format!(
                            "\"{}\"",
                            attr_value
                        );
                        println!("ATTRIBUTE VALUE: {}", attr_value);
                    },
                    TokenTree::Group(group) => {
                        attr_value = group.to_string();
                        println!("ATTRIBUTE VALUE: {}", attr_value);
                    },
                    token => {
                        return quote_spanned! {
                            token.span() =>
                            compile_error!("expected an attribute or '>'");
                        };
                    },
                }
            } else {
                return quote! { compile_error!("expected component to close") };
            }

            attributes.insert(attr_name, attr_value);
        }
    }

    let mut children = vec![];
    println!("PARSING CHILDREN");
    loop {
        match parse_child(iter) {
            IsChild::Some(child) => {
                children.push(child.to_string());
            },
            IsChild::Error(child) => {
                children.push(child.to_string());
            },
            _ => {
                break;
            },
        }
    }

    let end_name;
    if let Some(open) = iter.peek() {
        println!("PEEKING END NAME {}", open.to_string());
        if is_group {
            if open.to_string() != ">" {
                return quote_spanned! {
                    open.span() =>
                    compile_error!("expected a '>'");
                };
            }
        } else {
            match open {
                TokenTree::Ident(ident) => {
                    end_name = ident.to_string();
                    if end_name != name {
                        let span = ident.span();
                        return quote_spanned! {
                            span =>
                            compile_error!("expected a component name");
                        };
                    }

                    iter.next();
                    println!("COMPONENT NAME");
                },
                token => {
                    return quote_spanned! {
                        token.span() =>
                        compile_error!("expected a component name");
                    };
                }
            }
        }
    } else {
        return quote! { compile_error!("expected a component name") };
    }

    if let Some(close) = iter.peek() {
        if close.to_string() != ">" {
            return quote_spanned! {
                close.span() =>
                compile_error!("expected a closing bracket ('>')");
            };
        }

        iter.next();
    } else {
        return quote! { compile_error!("expected component to close") };
    }

    let mut element = String::new();

    if is_group {
        // compile children
        element.push_str("{let mut elem = fore::frontend::ElementGroup::new();");
        for child in children {
            element.push_str(
                &format!("elem.append_child({}.into());", child)
            );
        }
        element.push_str("elem}");
    } else {
        element.push('{');

        // compile attributes
        element.push_str("let mut attributes = std::collections::HashMap::new();");
        for (name, value) in attributes {
            element.push_str(
                &format!("attributes.insert(\"{}\".to_string(), {}.into());", name, value)
            );
        }

        // compile children
        element.push_str("let mut children: Vec<Box<dyn fore_frontend::DOMElement>> = vec![];");
        for child in children {
            element.push_str(
                &format!("children.push({}.into());", child)
            );
        }

        element.push_str(&format!("{}::new().render(attributes, children)}}", name));
    }

    TokenStream::from(element.parse::<proc_macro::TokenStream>().unwrap())
}

#[proc_macro]
pub fn rsx(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(stream);
    let mut iter = input.clone().into_iter().peekable();

    // The first character should be a `<`
    if let Some(open) = iter.next() {
        match open {
            TokenTree::Punct(punct) => {
                if punct.as_char() != '<' {
                    return proc_macro::TokenStream::from(
                        quote_spanned! {
                            punct.span() =>
                            compile_error!("expected an component declaration");
                        }
                    );
                }

                println!("COMPONENT OPEN");
            },
            token => {
                return proc_macro::TokenStream::from(
                    quote_spanned! {
                        token.span() =>
                        compile_error!("expected a component declaration");
                    }
                );
            }
        }
    } else {
        return proc_macro::TokenStream::from(
            quote! { compile_error!("expected a component declaration") }
        );
    }

    return proc_macro::TokenStream::from(parse_html_element(&mut iter));
}