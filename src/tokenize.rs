use std::char;
use std::fmt;
use std::str::Chars;

use crate::ast::{is_void_element, AttributeMap, Tag, TagKind, Token};

pub type Result<T> = std::result::Result<T, TokenizeError>;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    Malformed,
    UnexpectedChar(char, char), // (expected, actual)
    UnexpectedEOF,
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenizeError::Malformed => {
                write!(f, "syntactically malformed token found and ignored")
            }
            TokenizeError::UnexpectedChar(expected, actual) => {
                write!(f, "expected {} but got {}", expected, actual)
            }
            TokenizeError::UnexpectedEOF => write!(f, "unexpected EOF"),
        }
    }
}

impl std::error::Error for TokenizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            TokenizeError::Malformed => None,
            TokenizeError::UnexpectedChar(..) => None,
            TokenizeError::UnexpectedEOF => None,
        }
    }
}

pub struct Tokenizer<'a> {
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespaces();

            if self.is_eof() {
                break;
            }

            match self.read_token() {
                Ok(Token::Sgml) => continue,
                Ok(token) => tokens.push(token),
                Err(e) => {
                    if e == TokenizeError::Malformed {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn skip_whitespaces(&mut self) {
        while self.chars.next_if(|c| c.is_ascii_whitespace()).is_some() {
            continue;
        }
    }

    fn is_eof(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn read_token(&mut self) -> Result<Token> {
        if self.consume_char('<') {
            if self.consume_char('!') {
                self.read_sgml()
            } else {
                self.read_tag()
            }
        } else {
            self.read_text()
        }
    }

    fn read_sgml(&mut self) -> Result<Token> {
        loop {
            match self.chars.peek() {
                Some(c) => {
                    if *c == '>' {
                        self.chars.next();
                        break;
                    } else {
                        self.chars.next();
                        continue;
                    }
                }
                None => return Err(TokenizeError::UnexpectedEOF),
            }
        }

        Ok(Token::Sgml)
    }

    fn read_tag(&mut self) -> Result<Token> {
        let beginning_with_slash = self.consume_char('/');
        let name = self.read_tag_name()?;
        let (attributes, ending_with_slash) = self.read_attributes()?;

        if name.is_empty() {
            return Err(TokenizeError::Malformed);
        }

        if is_void_element(&name) {
            // <foo> and <foo/> are allowed for void element. </foo> or </foo/> are not.
            if beginning_with_slash {
                Err(TokenizeError::Malformed)
            } else {
                Ok(Token::Tag(Tag {
                    name,
                    attributes,
                    kind: TagKind::Void,
                }))
            }
        } else {
            // <foo> and </foo> are allowed for non-void element. <foo/> or </foo/> are not.
            if ending_with_slash {
                Err(TokenizeError::Malformed)
            } else if beginning_with_slash {
                Ok(Token::Tag(Tag {
                    name,
                    attributes,
                    kind: TagKind::Close,
                }))
            } else {
                Ok(Token::Tag(Tag {
                    name,
                    attributes,
                    kind: TagKind::Open,
                }))
            }
        }
    }

    fn read_tag_name(&mut self) -> Result<String> {
        let mut name = String::new();
        loop {
            match self.chars.peek() {
                Some(c) => {
                    if c.is_alphanumeric() {
                        name.push(*c);
                        self.chars.next();
                        continue;
                    } else {
                        break;
                    }
                }
                None => return Err(TokenizeError::UnexpectedEOF),
            }
        }

        Ok(name.to_ascii_lowercase())
    }

    fn read_attributes(&mut self) -> Result<(AttributeMap, bool)> {
        let mut attributes = AttributeMap::new();
        let mut ending_with_slash = false;

        loop {
            self.skip_whitespaces();

            if self.consume_char('/') {
                ending_with_slash = true;
                self.skip_whitespaces();
                self.expect_char('>')?;
                break;
            } else if self.consume_char('>') {
                break;
            }

            let name = self.read_attribute_name()?;
            let value = if self.consume_char('=') {
                self.read_attribute_value()?
            } else {
                name.clone()
            };

            attributes.insert(name, value);
        }

        Ok((attributes, ending_with_slash))
    }

    fn read_attribute_name(&mut self) -> Result<String> {
        let mut result = String::new();

        loop {
            match self.chars.peek() {
                Some(actual) => {
                    if actual.is_ascii_alphanumeric() || *actual == '-' || *actual == '_' {
                        result.push(*actual);
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                None => return Err(TokenizeError::UnexpectedEOF),
            }
        }

        Ok(result.to_lowercase())
    }

    fn read_attribute_value(&mut self) -> Result<String> {
        let mut result = String::new();

        self.expect_char('"')?;

        loop {
            match self.chars.peek() {
                Some(actual) => {
                    if *actual == '"' {
                        self.chars.next();
                        break;
                    } else {
                        result.push(*actual);
                        self.chars.next();
                        continue;
                    }
                }
                None => return Err(TokenizeError::UnexpectedEOF),
            }
        }

        Ok(result.to_lowercase())
    }

    fn read_text(&mut self) -> Result<Token> {
        let mut content = String::new();
        while let Some(c) = self.chars.next_if(|c| *c != '<') {
            content.push(c)
        }

        Ok(Token::Text(content))
    }

    fn consume_char(&mut self, expected: char) -> bool {
        self.chars.next_if(|c| *c == expected).is_some()
    }

    fn expect_char(&mut self, expected: char) -> Result<()> {
        match self.chars.next() {
            Some(actual) => {
                if actual == expected {
                    Ok(())
                } else {
                    Err(TokenizeError::UnexpectedChar(expected, actual))
                }
            }
            None => Err(TokenizeError::UnexpectedEOF),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_tokenizer_tokenize_empty() {
        let mut t = Tokenizer::new("");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(tokens, vec![]),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_only_doctype() {
        let mut t = Tokenizer::new("<!DOCTYPE html>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(tokens, vec![]),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    // #[test]
    // fn test_tokenizer_tokenize_doctype_without_bang() {
    //     let mut t = Tokenizer::new("<DOCTYPE html>");
    //     match t.tokenize() {
    //         Ok(tokens) => assert!(false, "Expected Err but got Ok: token = {:?}", tokens),
    //         Err(_) => assert!(true),
    //     }
    // }

    #[test]
    fn test_tokenizer_tokenize_doctype_and_open_element() {
        let mut t = Tokenizer::new("<!DOCTYPE html>\n<html>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: String::from("html"),
                    kind: TagKind::Open,
                    attributes: AttributeMap::new(),
                }),]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_doctype_and_close_element() {
        let mut t = Tokenizer::new("<!DOCTYPE html>\n</html>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: String::from("html"),
                    kind: TagKind::Close,
                    attributes: AttributeMap::new(),
                }),]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_open_and_close_tag() {
        let mut t = Tokenizer::new("<html></html>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![
                    Token::Tag(Tag {
                        name: String::from("html"),
                        kind: TagKind::Open,
                        attributes: AttributeMap::new(),
                    }),
                    Token::Tag(Tag {
                        name: String::from("html"),
                        kind: TagKind::Close,
                        attributes: AttributeMap::new(),
                    }),
                ]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_void_tag() {
        let mut t = Tokenizer::new("<hr/>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: String::from("hr"),
                    kind: TagKind::Void,
                    attributes: AttributeMap::new(),
                }),]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_uppercase_element() {
        let mut t = Tokenizer::new("<HTML>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: String::from("html"),
                    kind: TagKind::Open,
                    attributes: AttributeMap::new(),
                }),]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_closed_void_tag() {
        let mut t = Tokenizer::new("</foobar/>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(tokens, vec![]),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_only_opening_bracket() {
        let mut t = Tokenizer::new("<");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::UnexpectedEOF),
        }
    }

    // #[test]
    // fn test_tokenizer_tokenize_only_closing_bracket() {
    //     let mut t = Tokenizer::new(">");
    //     match t.tokenize() {
    //         Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
    //         Err(e) => assert_eq!(e, TokenizeError::UnexpectedChar('<', '>')),
    //     }
    // }

    #[test]
    fn test_tokenizer_tokenize_missing_tag_name_is_ignored() {
        let mut t = Tokenizer::new("<>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(tokens, vec![]),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    // #[test]
    // fn test_tokenizer_tokenize_missing_opening_bracket() {
    //     let mut t = Tokenizer::new("a>");
    //     match t.tokenize() {
    //         Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
    //         Err(e) => assert_eq!(e, TokenizeError::UnexpectedClosingBracket),
    //     }
    // }

    #[test]
    fn test_tokenizer_tokenize_missing_closing_bracket() {
        let mut t = Tokenizer::new("<a");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::UnexpectedEOF),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_text() {
        let mut t = Tokenizer::new("abcde");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(tokens, vec![Token::Text("abcde".to_string()),]),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_one_attribute() {
        let mut t = Tokenizer::new("<img src=\"hello.png\">");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "img".to_string(),
                    kind: TagKind::Void,
                    attributes: AttributeMap::from([("src".to_string(), "hello.png".to_string()),]),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_multiple_attributes() {
        let mut t = Tokenizer::new("<img src=\"hello.png\" width=\"300\">");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "img".to_string(),
                    kind: TagKind::Void,
                    attributes: AttributeMap::from([
                        ("src".to_string(), "hello.png".to_string()),
                        ("width".to_string(), "300".to_string()),
                    ]),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_one_boolean_attribute() {
        let mut t = Tokenizer::new("<input disabled>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "input".to_string(),
                    kind: TagKind::Void,
                    attributes: AttributeMap::from([(
                        "disabled".to_string(),
                        "disabled".to_string()
                    )]),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_void_tag_without_ending_slash() {
        for tag in vec![
            "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
            "source", "track", "wbr",
        ] {
            let source = format!("<{}>", tag);
            let mut t = Tokenizer::new(&source);
            match t.tokenize() {
                Ok(tokens) => assert_eq!(
                    tokens,
                    vec![Token::Tag(Tag {
                        name: tag.to_string(),
                        kind: TagKind::Void,
                        attributes: AttributeMap::new(),
                    })]
                ),
                Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
            }
        }
    }

    #[test]
    fn test_tokenizer_tokenize_void_element_with_ending_slash_are_allowed() {
        let mut t = Tokenizer::new("<br>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "br".to_string(),
                    kind: TagKind::Void,
                    attributes: AttributeMap::new(),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_void_element_without_ending_slash_are_allowed() {
        let mut t = Tokenizer::new("<br/>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "br".to_string(),
                    kind: TagKind::Void,
                    attributes: AttributeMap::new(),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_void_element_with_beginning_slash_is_ignored() {
        for tag in vec!["</br>", "</br/>"] {
            let mut t = Tokenizer::new(tag);
            match t.tokenize() {
                Ok(tokens) => assert_eq!(tokens, vec![]),
                Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
            }
        }
    }

    #[test]
    fn test_tokenizer_tokenize_non_void_element_with_beginning_slash_is_allowed() {
        let mut t = Tokenizer::new("<a>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "a".to_string(),
                    kind: TagKind::Open,
                    attributes: AttributeMap::new(),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_non_void_element_without_beginning_slash_is_allowed() {
        let mut t = Tokenizer::new("</a>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Tag(Tag {
                    name: "a".to_string(),
                    kind: TagKind::Close,
                    attributes: AttributeMap::new(),
                })]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_non_void_element_with_ending_slash_is_ignored() {
        for tag in vec!["<a/>", "</a/>"] {
            let mut t = Tokenizer::new(tag);
            match t.tokenize() {
                Ok(tokens) => assert_eq!(tokens, vec![]),
                Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
            }
        }
    }
}
