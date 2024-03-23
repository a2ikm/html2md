use std::char;
use std::fmt;
use std::str::Chars;

pub type Result<T> = std::result::Result<T, TokenizeError>;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    EOF,
    Malformed,
    NoTag,
    UnexpectedChar(char, char), // (expected, actual)
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenizeError::EOF => write!(f, "reached EOF"),
            TokenizeError::Malformed => write!(f, "malformed"),
            TokenizeError::NoTag => write!(f, "no tag"),
            TokenizeError::UnexpectedChar(expected, actual) => {
                write!(f, "expected {} but got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for TokenizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            TokenizeError::EOF => None,
            TokenizeError::NoTag => None,
            TokenizeError::Malformed => None,
            TokenizeError::UnexpectedChar(..) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Doctype,
    Tag(Tag),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum TagKind {
    Open,
    Close,
    Void,
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub kind: TagKind,
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

        let doctype = self.doctype()?;
        tokens.push(doctype);

        loop {
            match self.tag_or_text() {
                Ok(token) => tokens.push(token),
                Err(TokenizeError::EOF) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn consume_char(&mut self, expected: char) -> bool {
        match self.chars.next_if(|c| *c == expected) {
            Some(_) => true,
            None => false,
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<()> {
        match self.peek() {
            Some(actual) => {
                if *actual == expected {
                    self.next();
                    Ok(())
                } else {
                    Err(TokenizeError::UnexpectedChar(expected, *actual))
                }
            }
            None => Err(TokenizeError::EOF),
        }
    }

    fn peek_char(&mut self, expected: char) -> Result<bool> {
        match self.chars.peek() {
            Some(actual) => Ok(*actual == expected),
            None => Err(TokenizeError::EOF),
        }
    }

    fn doctype(&mut self) -> Result<Token> {
        for c in "<!DOCTYPE html>".chars() {
            self.expect_char(c)?;
        }
        Ok(Token::Doctype)
    }

    fn tag_name(&mut self) -> Result<String> {
        let mut tag = String::new();
        loop {
            match self.chars.next_if(|c| c.is_alphanumeric()) {
                Some(c) => tag.push(c),
                None => break,
            }
        }

        if tag.len() > 0 {
            Ok(tag.to_ascii_lowercase())
        } else {
            Err(TokenizeError::NoTag)
        }
    }

    fn tag(&mut self) -> Result<Token> {
        _ = self.expect_char('<')?;
        let beginning_with_slash = self.consume_char('/');
        let name = self.tag_name()?;
        let ending_with_slash = self.consume_char('/');
        _ = self.expect_char('>')?;

        if beginning_with_slash && ending_with_slash {
            Err(TokenizeError::Malformed)
        } else if beginning_with_slash {
            Ok(Token::Tag(Tag {
                name,
                kind: TagKind::Close,
            }))
        } else if ending_with_slash {
            Ok(Token::Tag(Tag {
                name,
                kind: TagKind::Void,
            }))
        } else {
            Ok(Token::Tag(Tag {
                name,
                kind: TagKind::Open,
            }))
        }
    }

    fn text(&mut self) -> Result<Token> {
        let mut content = String::new();
        loop {
            match self.chars.next_if(|c| *c != '<') {
                Some(c) => content.push(c),
                None => break,
            }
        }

        Ok(Token::Text(content))
    }

    fn tag_or_text(&mut self) -> Result<Token> {
        match self.peek_char('<') {
            Ok(true) => self.tag(),
            Ok(false) => self.text(),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_tokenize() {
        {
            let mut t = Tokenizer::new("<!DOCTYPE html>");
            match t.tokenize() {
                Ok(tokens) => assert_eq!(tokens, vec![Token::Doctype]),
                Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
            }
        }
        {
            let mut t =
                Tokenizer::new("<!DOCTYPE html><html><body><p>hello</p><hr/></body></html>");
            match t.tokenize() {
                Ok(tokens) => assert_eq!(
                    tokens,
                    vec![
                        Token::Doctype,
                        Token::Tag(Tag {
                            name: String::from("html"),
                            kind: TagKind::Open,
                        }),
                        Token::Tag(Tag {
                            name: String::from("body"),
                            kind: TagKind::Open,
                        }),
                        Token::Tag(Tag {
                            name: String::from("p"),
                            kind: TagKind::Open,
                        }),
                        Token::Text(String::from("hello")),
                        Token::Tag(Tag {
                            name: String::from("p"),
                            kind: TagKind::Close,
                        }),
                        Token::Tag(Tag {
                            name: String::from("hr"),
                            kind: TagKind::Void,
                        }),
                        Token::Tag(Tag {
                            name: String::from("body"),
                            kind: TagKind::Close,
                        }),
                        Token::Tag(Tag {
                            name: String::from("html"),
                            kind: TagKind::Close,
                        }),
                    ]
                ),
                Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
            }
        }
    }

    #[test]
    fn test_tokenizer_doctype() {
        {
            let mut t = Tokenizer::new("<!DOCTYPE html>");
            match t.doctype() {
                Ok(token) => assert_eq!(Token::Doctype, token),
                Err(e) => assert!(false, "Expected Ok but got Err: error = {}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<DOCTYPE html>");
            match t.doctype() {
                Ok(token) => assert!(false, "Expected Err but got Ok: token = {:?}", token),
                Err(_) => assert!(true),
            }
        }
        {
            let mut t = Tokenizer::new("");
            match t.doctype() {
                Ok(token) => assert!(false, "Expected Err(EOF) but got Ok: token = {:?}", token),
                Err(e) => assert_eq!(TokenizeError::EOF, e),
            }
        }
        {
            let mut t = Tokenizer::new("<");
            match t.doctype() {
                Ok(token) => assert!(false, "Expected Err(EOF) but got Ok: token = {:?}", token),
                Err(e) => assert_eq!(TokenizeError::EOF, e),
            }
        }
        {
            let mut t = Tokenizer::new(">");
            match t.doctype() {
                Ok(token) => assert!(
                    false,
                    "Expected Err(UnexpectedChar) but got Ok: token = {:?}",
                    token
                ),
                Err(e) => assert_eq!(TokenizeError::UnexpectedChar('<', '>'), e),
            }
        }
    }

    #[test]
    fn test_tokenizer_tag() {
        {
            let mut t = Tokenizer::new("<a>");
            match t.tag() {
                Ok(Token::Tag(tag)) => assert_eq!(
                    tag,
                    Tag {
                        name: String::from("a"),
                        kind: TagKind::Open,
                    }
                ),
                Ok(token) => assert!(false, "Expected Token::Tag, but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok but got Err: error = {}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<A>");
            match t.tag() {
                Ok(Token::Tag(tag)) => assert_eq!(
                    tag,
                    Tag {
                        name: String::from("a"),
                        kind: TagKind::Open,
                    }
                ),
                Ok(token) => assert!(false, "Expected Token::Tag, but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok but got Err: error = {}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<table>");
            match t.tag() {
                Ok(Token::Tag(tag)) => assert_eq!(
                    tag,
                    Tag {
                        name: String::from("table"),
                        kind: TagKind::Open,
                    }
                ),
                Ok(token) => assert!(false, "Expected Token::Tag, but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok but got Err: error = {}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<>");
            match t.tag() {
                Ok(token) => assert!(
                    false,
                    "Expected Err(TokenizeError::NoTag), but got {:?}",
                    token
                ),
                Err(e) => assert_eq!(e, TokenizeError::NoTag),
            }
        }
        {
            let mut t = Tokenizer::new("a>");
            match t.tag() {
                Ok(token) => assert!(
                    false,
                    "Expected Err(TokenizeError::UnexpectedChar), but got {:?}",
                    token
                ),
                Err(e) => assert_eq!(e, TokenizeError::UnexpectedChar('<', 'a')),
            }
        }
        {
            let mut t = Tokenizer::new("<a");
            match t.tag() {
                Ok(token) => assert!(
                    false,
                    "Expected Err(TokenizeError::NoTag), but got {:?}",
                    token
                ),
                Err(e) => assert_eq!(e, TokenizeError::EOF),
            }
        }
    }

    #[test]
    fn test_tokenizer_text() {
        {
            let mut t = Tokenizer::new("");
            match t.text() {
                Ok(Token::Text(content)) => assert_eq!(content, ""),
                Ok(token) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", e),
            }
        }
        {
            let mut t = Tokenizer::new("abcde");
            match t.text() {
                Ok(Token::Text(content)) => assert_eq!(content, "abcde"),
                Ok(token) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<");
            match t.text() {
                Ok(Token::Text(content)) => assert_eq!(content, ""),
                Ok(token) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", e),
            }
        }
    }

    #[test]
    fn test_tokenizer_tag_or_text() {
        {
            let mut t = Tokenizer::new("abcde");
            match t.tag_or_text() {
                Ok(Token::Text(content)) => assert_eq!(content, "abcde"),
                Ok(token) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok(Token::Text(\"\") but got {:?}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<a>");
            match t.tag_or_text() {
                Ok(Token::Tag(tag)) => assert_eq!(
                    tag,
                    Tag {
                        name: String::from("a"),
                        kind: TagKind::Open,
                    }
                ),
                Ok(token) => assert!(false, "Expected Ok(Token::Tag(...) but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok(Token::Tag(...) but got {:?}", e),
            }
        }
        {
            let mut t = Tokenizer::new("");
            match t.tag_or_text() {
                Ok(token) => assert!(
                    false,
                    "Expected Err(TokenizerError::EOF) but got Ok({:?})",
                    token
                ),
                Err(e) => assert_eq!(e, TokenizeError::EOF),
            }
        }
    }
}
