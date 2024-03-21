use std::char;
use std::fmt;
use std::str::Chars;

pub type Result<T> = std::result::Result<T, TokenizeError>;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    EOF,
    UnexpectedChar(char, char), // (expected, actual)
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenizeError::EOF => write!(f, "reached EOF"),
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
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            TokenizeError::UnexpectedChar(..) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Doctype,
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

    fn doctype(&mut self) -> Result<Token> {
        for c in "<!DOCTYPE html>".chars() {
            self.expect_char(c)?;
        }
        Ok(Token::Doctype)
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut t = Tokenizer::new(source);

    let doctype = t.doctype()?;
    tokens.push(doctype);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_doctype() {
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
}
