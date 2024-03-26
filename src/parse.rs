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
pub enum Node<'a> {
    Element(Element<'a>),
    Text(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element<'a> {
    pub tag: &'a str,
    pub children: Vec<Node<'a>>,
}

impl<'a> Element<'a> {
    pub fn new(tag: &'a str) -> Self {
        Self {
            tag,
            children: Vec::new(),
        }
    }

    pub fn new_with_children(tag: &'a str, children: Vec<Node<'a>>) -> Self {
        Self { tag, children }
    }
}

pub fn parse<'a>(tokens: &'a Vec<tokenize::Token>) -> Result<Node<'a>> {
    let mut it = tokens.iter().peekable();
    html(&mut it)
}

fn expect_open_tag_with_name<'a>(
    tokens: &mut Peekable<Iter<'a, tokenize::Token>>,
    name: &str,
) -> Result<&'a tokenize::Tag> {
    match tokens.next() {
        Some(tokenize::Token::Tag(tag)) => {
            if tag.name == name && tag.kind == tokenize::TagKind::Open {
                Ok(tag)
            } else {
                Err(ParseError::UnexpectedToken)
            }
        }
        Some(_) => Err(ParseError::UnexpectedToken),
        None => Err(ParseError::UnexpectedEOF),
    }
}

fn expect_close_tag_with_name<'a>(
    tokens: &mut Peekable<Iter<'a, tokenize::Token>>,
    name: &str,
) -> Result<&'a tokenize::Tag> {
    match tokens.next() {
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

fn expect_element<'a>(tokens: &mut Peekable<Iter<'a, tokenize::Token>>) -> Result<Node<'a>> {
    match tokens.next() {
        Some(tokenize::Token::Tag(tag)) => match tag.kind {
            tokenize::TagKind::Open => {
                let children = element_or_text_nodes(tokens)?;
                let _close_tag = expect_close_tag_with_name(tokens, &tag.name)?;
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

fn expect_text<'a>(tokens: &mut Peekable<Iter<'a, tokenize::Token>>) -> Result<Node<'a>> {
    match tokens.next() {
        Some(tokenize::Token::Text(content)) => Ok(Node::Text(content)),
        Some(_) => Err(ParseError::UnexpectedToken),
        None => Err(ParseError::UnexpectedEOF),
    }
}

fn element_or_text_nodes<'a>(
    tokens: &mut Peekable<Iter<'a, tokenize::Token>>,
) -> Result<Vec<Node<'a>>> {
    let mut nodes = Vec::new();

    loop {
        match tokens.peek() {
            Some(tokenize::Token::Tag(tag)) => match tag.kind {
                tokenize::TagKind::Open | tokenize::TagKind::Void => {
                    let node = expect_element(tokens)?;
                    nodes.push(node);
                }
                tokenize::TagKind::Close => break,
            },
            Some(tokenize::Token::Text(_content)) => {
                let node = expect_text(tokens)?;
                nodes.push(node);
            }
            Some(_) => return Err(ParseError::UnexpectedToken),
            None => return Err(ParseError::UnexpectedEOF),
        }
    }

    Ok(nodes)
}

fn element_nodes<'a>(tokens: &mut Peekable<Iter<'a, tokenize::Token>>) -> Result<Vec<Node<'a>>> {
    let mut nodes = Vec::new();

    loop {
        match tokens.peek() {
            Some(tokenize::Token::Tag(tag)) => match tag.kind {
                tokenize::TagKind::Open | tokenize::TagKind::Void => {
                    let node = expect_element(tokens)?;
                    nodes.push(node);
                }
                _ => break,
            },
            Some(_) => break,
            None => return Err(ParseError::UnexpectedEOF),
        }
    }

    Ok(nodes)
}

fn html<'a>(tokens: &mut Peekable<Iter<'a, tokenize::Token>>) -> Result<Node<'a>> {
    let open_tag = expect_open_tag_with_name(tokens, "html")?;
    let head = head(tokens)?;
    let body = body(tokens)?;
    let _close_tag = expect_close_tag_with_name(tokens, "html")?;

    Ok(Node::Element(Element::new_with_children(
        &open_tag.name,
        vec![head, body],
    )))
}

fn head<'a>(tokens: &mut Peekable<Iter<'a, tokenize::Token>>) -> Result<Node<'a>> {
    let open_tag = expect_open_tag_with_name(tokens, "head")?;
    let children = element_nodes(tokens)?;
    let _close_tag = expect_close_tag_with_name(tokens, "head")?;

    Ok(Node::Element(Element::new_with_children(
        &open_tag.name,
        children,
    )))
}

fn body<'a>(tokens: &mut Peekable<Iter<'a, tokenize::Token>>) -> Result<Node<'a>> {
    let open_tag = expect_open_tag_with_name(tokens, "body")?;
    let children = element_or_text_nodes(tokens)?;
    let _close_tag = expect_close_tag_with_name(tokens, "body")?;

    Ok(Node::Element(Element::new_with_children(
        &open_tag.name,
        children,
    )))
}
