use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Token {
    Sgml,
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

impl Node {
    pub fn is_list_element(&self) -> bool {
        match self {
            Self::Element(element) => element.is_list_element(),
            Self::Text(_) => false,
        }
    }
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

    fn css_classes(&self) -> Vec<String> {
        match self.attributes.get("class") {
            Some(value) => value
                .as_str()
                .split(' ')
                .map(|s| s.trim().to_string())
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn list_depth(&self) -> usize {
        let found = self
            .css_classes()
            .iter()
            .filter(|class| class.contains('-'))
            .map(|class| {
                let n = class.split('-').last().unwrap();
                usize::from_str_radix(n, 10)
            })
            .filter(|n| n.is_ok())
            .last();
        if let Some(ok) = found {
            ok.unwrap()
        } else {
            0
        }
    }

    pub fn is_list_element(&self) -> bool {
        self.tag == "ul" || self.tag == "ol"
    }
}

pub fn is_void_element(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

pub fn is_block_element(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "canvas"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hr"
            | "li"
            | "main"
            | "nav"
            | "noscript"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "tfoot"
            | "ul"
            | "video"
            | "html2md:successive-lists-wrapper"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_list_depth() {
        {
            let element = Element::new(
                "ul",
                &AttributeMap::from([("class".to_string(), "foo-2".to_string())]),
            );
            assert_eq!(element.list_depth(), 2)
        }
        {
            let element = Element::new(
                "ul",
                &AttributeMap::from([("class".to_string(), "bar foo-2".to_string())]),
            );
            assert_eq!(element.list_depth(), 2)
        }
        {
            let element = Element::new(
                "ul",
                &AttributeMap::from([("class".to_string(), "foo-2 bar".to_string())]),
            );
            assert_eq!(element.list_depth(), 2)
        }
        {
            let element = Element::new(
                "ul",
                &AttributeMap::from([("class".to_string(), "foo-2 bar-3 buz-4".to_string())]),
            );
            assert_eq!(element.list_depth(), 4)
        }
        {
            let element = Element::new(
                "ul",
                &AttributeMap::from([("class".to_string(), "".to_string())]),
            );
            assert_eq!(element.list_depth(), 0)
        }
        {
            let element = Element::new(
                "ul",
                &AttributeMap::from([("class".to_string(), "foo-bar".to_string())]),
            );
            assert_eq!(element.list_depth(), 0)
        }
    }
}
