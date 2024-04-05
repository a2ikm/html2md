use crate::tokenize;
use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedEOF => write!(f, "unexpected EOF"),
            ParseError::UnexpectedToken => write!(f, "unexpected token"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ParseError::UnexpectedEOF => None,
            ParseError::UnexpectedToken => None,
        }
    }
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
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            children: Vec::new(),
        }
    }

    pub fn new_with_children(tag: &str, children: Vec<Node>) -> Self {
        Self {
            tag: tag.to_string(),
            children,
        }
    }
}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, tokenize::Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<tokenize::Token>) -> Self {
        let it = tokens.iter().peekable();
        Self { tokens: it }
    }

    pub fn parse(&mut self) -> Result<Node> {
        self.expect_element()
    }

    fn expect_close_tag_with_name(&mut self, name: &str) -> Result<&'a tokenize::Tag> {
        match self.tokens.next() {
            Some(tokenize::Token::Tag(tag)) => {
                if tag.name == name && tag.kind == tokenize::TagKind::Close {
                    Ok(tag)
                } else {
                    Err(ParseError::UnexpectedToken)
                }
            }
            Some(_) => Err(ParseError::UnexpectedToken),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_element(&mut self) -> Result<Node> {
        match self.tokens.next() {
            Some(tokenize::Token::Tag(tag)) => match tag.kind {
                tokenize::TagKind::Open => {
                    let children = self.element_or_text_nodes()?;
                    let _close_tag = self.expect_close_tag_with_name(&tag.name)?;
                    Ok(Node::Element(Element::new_with_children(
                        &tag.name, children,
                    )))
                }
                tokenize::TagKind::Void => Ok(Node::Element(Element::new(&tag.name))),
                tokenize::TagKind::Close => Err(ParseError::UnexpectedToken),
            },
            Some(_) => Err(ParseError::UnexpectedToken),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_text(&mut self) -> Result<Node> {
        match self.tokens.next() {
            Some(tokenize::Token::Text(content)) => Ok(Node::Text(content.to_string())),
            Some(_) => Err(ParseError::UnexpectedToken),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn element_or_text_nodes(&mut self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();

        loop {
            match self.tokens.peek() {
                Some(tokenize::Token::Tag(tag)) => match tag.kind {
                    tokenize::TagKind::Open | tokenize::TagKind::Void => {
                        let node = self.expect_element()?;
                        nodes.push(node);
                    }
                    tokenize::TagKind::Close => break,
                },
                Some(tokenize::Token::Text(_content)) => {
                    let node = self.expect_text()?;
                    nodes.push(node);
                }
                Some(_) => return Err(ParseError::UnexpectedToken),
                None => return Err(ParseError::UnexpectedEOF),
            }
        }

        Ok(nodes)
    }
}
