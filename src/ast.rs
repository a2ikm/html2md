use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Token {
    SGML,
    Tag(Tag),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum TagKind {
    Open,
    Close,
    Void,
}

pub type AttributeMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub kind: TagKind,
    pub attributes: AttributeMap,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub tag: String,
    pub children: Vec<Node>,
    pub attributes: AttributeMap,
}

impl Element {
    pub fn new(tag: &str, attributes: &AttributeMap) -> Self {
        Self {
            tag: tag.to_string(),
            children: Vec::new(),
            attributes: attributes.clone(),
        }
    }

    pub fn new_with_children(tag: &str, attributes: &AttributeMap, children: Vec<Node>) -> Self {
        Self {
            tag: tag.to_string(),
            children,
            attributes: attributes.clone(),
        }
    }
}

pub fn is_void_element(tag_name: &str) -> bool {
    match tag_name {
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta"
        | "param" | "source" | "track" | "wbr" => true,
        _ => false,
    }
}

pub fn is_block_element(tag_name: &str) -> bool {
    match tag_name {
        "address" | "article" | "aside" | "blockquote" | "canvas" | "dd" | "div" | "dl" | "dt"
        | "fieldset" | "figcaption" | "figure" | "footer" | "form" | "h1" | "h2" | "h3" | "h4"
        | "h5" | "h6" | "header" | "hr" | "li" | "main" | "nav" | "noscript" | "ol" | "p"
        | "pre" | "section" | "table" | "tfoot" | "ul" | "video" => true,
        _ => false,
    }
}
