use std::char;
use std::fmt;
use std::str::Chars;

pub type Result<T> = std::result::Result<T, TokenizeError>;

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    Malformed,
    NoTag,
    UnexpectedChar(char, char), // (expected, actual)
    UnexpectedEOF,
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenizeError::Malformed => write!(f, "malformed"),
            TokenizeError::NoTag => write!(f, "no tag"),
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
            TokenizeError::NoTag => None,
            TokenizeError::Malformed => None,
            TokenizeError::UnexpectedChar(..) => None,
            TokenizeError::UnexpectedEOF => None,
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
            self.skip_whitespaces();

            if self.is_eof() {
                break;
            }

            match self.tag_or_text() {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }

    fn skip_whitespaces(&mut self) -> () {
        loop {
            match self.chars.next_if(|c| c.is_ascii_whitespace()) {
                Some(_) => continue,
                None => break,
            }
        }
    }

    fn is_eof(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn consume_char(&mut self, expected: char) -> bool {
        match self.chars.next_if(|c| *c == expected) {
            Some(_) => true,
            None => false,
        }
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

    fn peek_char(&mut self, expected: char) -> Result<bool> {
        match self.chars.peek() {
            Some(actual) => Ok(*actual == expected),
            None => Err(TokenizeError::UnexpectedEOF),
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
    fn test_tokenizer_tokenize_empty() {
        let mut t = Tokenizer::new("");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::UnexpectedEOF),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_only_doctype() {
        let mut t = Tokenizer::new("<!DOCTYPE html>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(tokens, vec![Token::Doctype]),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_doctype_without_bang() {
        let mut t = Tokenizer::new("<DOCTYPE html>");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok: token = {:?}", tokens),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_doctype_and_open_element() {
        let mut t = Tokenizer::new("<!DOCTYPE html>\n<html>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![
                    Token::Doctype,
                    Token::Tag(Tag {
                        name: String::from("html"),
                        kind: TagKind::Open,
                    }),
                ]
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
                vec![
                    Token::Doctype,
                    Token::Tag(Tag {
                        name: String::from("html"),
                        kind: TagKind::Close,
                    }),
                ]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_open_and_close_tag() {
        let mut t = Tokenizer::new("<!DOCTYPE html><html></html>");
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
                        name: String::from("html"),
                        kind: TagKind::Close,
                    }),
                ]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_void_tag() {
        let mut t = Tokenizer::new("<!DOCTYPE html><hr/>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![
                    Token::Doctype,
                    Token::Tag(Tag {
                        name: String::from("hr"),
                        kind: TagKind::Void,
                    }),
                ]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_uppercase_element() {
        let mut t = Tokenizer::new("<!DOCTYPE html><HTML>");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![
                    Token::Doctype,
                    Token::Tag(Tag {
                        name: String::from("html"),
                        kind: TagKind::Open,
                    }),
                ]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_closed_void_tag() {
        let mut t = Tokenizer::new("<!DOCTYPE html></foobar/>");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err(Malformed) but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::Malformed),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_only_opening_bracket() {
        let mut t = Tokenizer::new("<!DOCTYPE html><");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::NoTag),
        }
    }

    // #[test]
    // fn test_tokenizer_tokenize_only_closing_bracket() {
    //     let mut t = Tokenizer::new("<!DOCTYPE html>>");
    //     match t.tokenize() {
    //         Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
    //         Err(e) => assert_eq!(e, TokenizeError::UnexpectedChar('<', '>')),
    //     }
    // }

    #[test]
    fn test_tokenizer_tokenize_missing_tag_name() {
        let mut t = Tokenizer::new("<!DOCTYPE html><>");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::NoTag),
        }
    }

    // #[test]
    // fn test_tokenizer_tokenize_missing_opening_bracket() {
    //     let mut t = Tokenizer::new("<!DOCTYPE html>a>");
    //     match t.tokenize() {
    //         Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
    //         Err(e) => assert_eq!(e, TokenizeError::UnexpectedClosingBracket),
    //     }
    // }

    #[test]
    fn test_tokenizer_tokenize_missing_closing_bracket() {
        let mut t = Tokenizer::new("<!DOCTYPE html><a");
        match t.tokenize() {
            Ok(tokens) => assert!(false, "Expected Err but got Ok({:?})", tokens),
            Err(e) => assert_eq!(e, TokenizeError::UnexpectedEOF),
        }
    }

    #[test]
    fn test_tokenizer_tokenize_text() {
        let mut t = Tokenizer::new("<!DOCTYPE html>abcde");
        match t.tokenize() {
            Ok(tokens) => assert_eq!(
                tokens,
                vec![Token::Doctype, Token::Text("abcde".to_string()),]
            ),
            Err(e) => assert!(false, "Expected Ok but got Err({:?})", e),
        }
    }
}
