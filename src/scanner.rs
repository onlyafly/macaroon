use std::iter::Peekable;
use std::str::Chars;
use tokens::Token;

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &str) -> Scanner {
        Scanner {
            input: input.chars().peekable(),
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('-') => {
                if let Some(&ch) = self.peek_char() {
                    if ch.is_numeric() {
                        Token::Number(self.scan_number('-'))
                    } else {
                        Token::Symbol(self.scan_symbol('-'))
                    }
                } else {
                    Token::Error
                }
            }
            Some('^') => Token::Caret,
            Some('\'') => Token::SingleQuote,
            Some('\\') => {
                if let Some(ch) = self.read_char() {
                    Token::Char(ch.to_string())
                } else {
                    Token::Error
                }
            }

            // TODO
            //  1. Single line comments
            //  2. Floating point numbers
            //  3. Multiline comments
            //  4. String literals
            //  5. Tracking location of errors
            Some(ch @ _) => {
                if ch.is_numeric() {
                    Token::Number(self.scan_number(ch))
                } else if is_symbolic(ch) {
                    Token::Symbol(self.scan_symbol(ch))
                } else {
                    Token::Error
                }
            }

            None => Token::EndOfFile,
        }
    }

    fn scan_number(&mut self, first: char) -> String {
        let mut number = String::new();

        number.push(first);

        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap()); // TODO: unwrap()
        }

        number
    }

    fn scan_symbol(&mut self, first: char) -> String {
        let mut symbol_text = String::new();
        symbol_text.push(first);

        while self.peek_is_symbolic() {
            symbol_text.push(self.read_char().unwrap());
        }

        symbol_text
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_is_symbolic(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_symbolic(ch),
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }

            // TODO: add newline tracking logic here to count which line we are on

            self.read_char();
        }
    }
}

fn is_symbolic(ch: char) -> bool {
    // NOTE: Don't ever allow these characters: [ ] { } ( ) " , ' ` : ; # | \ ~
    ch.is_alphabetic()
        || ch.is_numeric()
        || ch == '?'
        || ch == '+'
        || ch == '*'
        || ch == '/'
        || ch == '='
        || ch == '<'
        || ch == '>'
        || ch == '!'
        || ch == '&'
        || ch == '.'
        || ch == '-'
        || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_numbers() {
        let mut s = Scanner::new("1 2 3");
        assert_eq!(s.next(), Token::Number("1".to_string()));
        assert_eq!(s.next(), Token::Number("2".to_string()));
        assert_eq!(s.next(), Token::Number("3".to_string()));
        assert_eq!(s.next(), Token::EndOfFile);
    }

    #[test]
    fn test_parens_and_numbers() {
        let mut s = Scanner::new("((1))");
        assert_eq!(s.next(), Token::LeftParen);
        assert_eq!(s.next(), Token::LeftParen);
        assert_eq!(s.next(), Token::Number("1".to_string()));
        assert_eq!(s.next(), Token::RightParen);
        assert_eq!(s.next(), Token::RightParen);
        assert_eq!(s.next(), Token::EndOfFile);
    }

    #[test]
    fn test_symbols_and_numbers_with_minus_sign() {
        let mut s = Scanner::new("- -aa -123");
        assert_eq!(s.next(), Token::Symbol("-".to_string()));
        assert_eq!(s.next(), Token::Symbol("-aa".to_string()));
        assert_eq!(s.next(), Token::Number("-123".to_string()));
        assert_eq!(s.next(), Token::EndOfFile);
    }

    #[test]
    fn test_symbols() {
        let mut s = Scanner::new("a b123 cAcZ ? + - * / = < > ! & _ . <aA1+-*/=<>!&_");
        assert_eq!(s.next(), Token::Symbol("a".to_string()));
        assert_eq!(s.next(), Token::Symbol("b123".to_string()));
        assert_eq!(s.next(), Token::Symbol("cAcZ".to_string()));
        assert_eq!(s.next(), Token::Symbol("?".to_string()));
        assert_eq!(s.next(), Token::Symbol("+".to_string()));
        assert_eq!(s.next(), Token::Symbol("-".to_string()));
        assert_eq!(s.next(), Token::Symbol("*".to_string()));
        assert_eq!(s.next(), Token::Symbol("/".to_string()));
        assert_eq!(s.next(), Token::Symbol("=".to_string()));
        assert_eq!(s.next(), Token::Symbol("<".to_string()));
        assert_eq!(s.next(), Token::Symbol(">".to_string()));
        assert_eq!(s.next(), Token::Symbol("!".to_string()));
        assert_eq!(s.next(), Token::Symbol("&".to_string()));
        assert_eq!(s.next(), Token::Symbol("_".to_string()));
        assert_eq!(s.next(), Token::Symbol(".".to_string()));
        assert_eq!(s.next(), Token::Symbol("<aA1+-*/=<>!&_".to_string()));
        assert_eq!(s.next(), Token::EndOfFile);
    }

    #[test]
    fn test_miscellaneous() {
        let mut s = Scanner::new(r"^ '");
        assert_eq!(s.next(), Token::Caret);
        assert_eq!(s.next(), Token::SingleQuote);
        assert_eq!(s.next(), Token::EndOfFile);
    }

    #[test]
    fn test_chars() {
        let mut s = Scanner::new(r"\a");
        assert_eq!(s.next(), Token::Char("a".to_string()));
        assert_eq!(s.next(), Token::EndOfFile);
    }

    #[test]
    fn test_errors() {
        let mut s = Scanner::new(r"\");
        assert_eq!(s.next(), Token::Error);
    }

    #[test]
    fn test_quoting() {
        let mut s = Scanner::new(r"'a");
        assert_eq!(s.next(), Token::SingleQuote);
        assert_eq!(s.next(), Token::Symbol("a".to_string()));
        assert_eq!(s.next(), Token::EndOfFile);
    }
}
