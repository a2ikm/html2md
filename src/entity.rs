use std::char;
use std::str::Chars;

pub struct Translator<'a> {
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Translator<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            chars: name.chars().peekable(),
        }
    }

    pub fn translate(&mut self) -> String {
        if self.consume_char('#') {
            if self.consume_char('x') || self.consume_char('X') {
                self.translate_hexadecimal_reference()
            } else {
                self.translate_decimal_reference()
            }
        } else {
            self.translate_reserved_reference()
        }
    }

    fn translate_hexadecimal_reference(&mut self) -> String {
        match u32::from_str_radix(&self.rest(), 16) {
            Ok(code) => match char::from_u32(code) {
                Some(c) => c.to_string(),
                None => String::new(),
            },
            Err(_) => String::new(),
        }
    }

    fn translate_decimal_reference(&mut self) -> String {
        match u32::from_str_radix(&self.rest(), 10) {
            Ok(code) => match char::from_u32(code) {
                Some(c) => c.to_string(),
                None => String::new(),
            },
            Err(_) => String::new(),
        }
    }

    fn translate_reserved_reference(&mut self) -> String {
        format!("&{};", self.rest())
    }

    fn consume_char(&mut self, expected: char) -> bool {
        match self.chars.next_if(|c| *c == expected) {
            Some(_) => true,
            None => false,
        }
    }

    fn rest(&mut self) -> String {
        let mut result = String::new();
        loop {
            match self.chars.next() {
                Some(c) => result.push(c),
                None => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn translate(name: &str) -> String {
        Translator::new(name).translate()
    }

    #[test]
    fn test_translate_reserved() {
        assert_eq!(translate("nbsp"), "&nbsp;");
    }

    #[test]
    fn test_translate_decimal() {
        assert_eq!(translate("#35"), "#");
        assert_eq!(translate("#1234"), "Ӓ");
    }

    #[test]
    fn test_translate_hexadecimal() {
        assert_eq!(translate("#xd06"), "ആ");
        assert_eq!(translate("#Xcab"), "ಫ");
    }
}
