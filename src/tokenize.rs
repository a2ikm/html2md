use std::char;
use std::fmt;
use std::str::Chars;

pub type Result<T> = std::result::Result<T, TokenizeError>;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    EOF,
    NoTag,
    UnexpectedChar(char, char), // (expected, actual)
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenizeError::EOF => write!(f, "reached EOF"),
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
pub struct Tag {
    name: String,
}

struct Tokenizer<'a> {
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
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
            Ok(tag)
        } else {
            Err(TokenizeError::NoTag)
        }
    }

    fn tag(&mut self) -> Result<Token> {
        _ = self.expect_char('<')?;
        let name = self.tag_name()?;
        _ = self.expect_char('>')?;
        Ok(Token::Tag(Tag { name: name }))
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

pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut t = Tokenizer::new(source);

    let doctype = t.doctype()?;
    tokens.push(doctype);

    loop {
        match t.tag_or_text() {
            Ok(token) => tokens.push(token),
            Err(TokenizeError::EOF) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        {
            match tokenize("<!DOCTYPE html>") {
                Ok(tokens) => assert_eq!(tokens, vec![Token::Doctype]),
                Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
            }
        }
        {
            match tokenize("<!DOCTYPE html><html><body><p>hello") {
                Ok(tokens) => assert_eq!(
                    tokens,
                    vec![
                        Token::Doctype,
                        Token::Tag(Tag {
                            name: String::from("html")
                        }),
                        Token::Tag(Tag {
                            name: String::from("body")
                        }),
                        Token::Tag(Tag {
                            name: String::from("p")
                        }),
                        Token::Text(String::from("hello")),
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
                Ok(Token::Tag(tag)) => assert_eq!("a", tag.name),
                Ok(token) => assert!(false, "Expected Token::Tag, but got {:?}", token),
                Err(e) => assert!(false, "Expected Ok but got Err: error = {}", e),
            }
        }
        {
            let mut t = Tokenizer::new("<table>");
            match t.tag() {
                Ok(Token::Tag(tag)) => assert_eq!("table", tag.name),
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
                Ok(Token::Tag(tag)) => assert_eq!("a", tag.name),
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
