use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;

use crate::ast::{Element, Node, Tag, TagKind, Token};

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

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let it = tokens.iter().peekable();
        Self { tokens: it }
    }

    pub fn parse(&mut self) -> Result<Node> {
        self.expect_element()
    }

    fn expect_close_tag_with_name(&mut self, name: &str) -> Result<&'a Tag> {
        match self.tokens.next() {
            Some(Token::Tag(tag)) => {
                if tag.name == name && tag.kind == TagKind::Close {
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
            Some(Token::Tag(tag)) => match tag.kind {
                TagKind::Open => {
                    let children = self.element_or_text_nodes()?;
                    let _close_tag = self.expect_close_tag_with_name(&tag.name)?;
                    Ok(Node::Element(Element::new_with_children(
                        &tag.name,
                        &tag.attributes,
                        children,
                    )))
                }
                TagKind::Void => Ok(Node::Element(Element::new(&tag.name, &tag.attributes))),
                TagKind::Close => Err(ParseError::UnexpectedToken),
            },
            Some(_) => Err(ParseError::UnexpectedToken),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_text(&mut self) -> Result<Node> {
        match self.tokens.next() {
            Some(Token::Text(content)) => Ok(Node::Text(content.to_string())),
            Some(_) => Err(ParseError::UnexpectedToken),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn element_or_text_nodes(&mut self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();

        loop {
            match self.tokens.peek() {
                Some(Token::Tag(tag)) => match tag.kind {
                    TagKind::Open | TagKind::Void => {
                        let node = self.expect_element()?;
                        nodes.push(node);
                    }
                    TagKind::Close => break,
                },
                Some(Token::Text(_content)) => {
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
