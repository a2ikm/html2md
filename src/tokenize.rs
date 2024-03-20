use std::{error::Error, str::Chars};

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

    fn expect_char(&mut self, expected: char) -> Result<(), Box<dyn Error>> {
        match self.peek() {
            Some(ch) => {
                if *ch == expected {
                    self.next();
                    Ok(())
                } else {
                    Err(format!("expected {} but got {}", expected, ch).into())
                }
            }
            None => Err("EOF".into()),
        }
    }

    fn doctype(&mut self) -> Result<Token, Box<dyn Error>> {
        for c in "<!DOCTYPE html>".chars() {
            self.expect_char(c)?;
        }
        Ok(Token::Doctype)
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn Error>> {
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
    }
}
