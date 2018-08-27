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
                        Token::Number(self.scan_number(ch, true))
                    } else {
                        Token::Symbol(self.scan_symbol('-'))
                    }
                } else {
                    Token::Error
                }
            }

            /*
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }

            Some('+') => Token::Plus,

            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LowerThan,
            Some('>') => Token::GreaterThan,
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            */
            Some(ch @ _) => {
                /*
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    token::lookup_ident(&literal)
                } else*/
                if ch.is_numeric() {
                    Token::Number(self.scan_number(ch, false))
                } else {
                    Token::Error
                }
            }

            // Handle EOF
            None => Token::EndOfFile,
        }
    }

    fn scan_number(&mut self, first: char, is_negative: bool) -> String {
        let mut number = String::new();

        if is_negative {
            number.push('-');
        }
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
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_symbolic() {
            ident.push(self.read_char().unwrap());
        }

        ident
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
    ch.is_alphabetic() || ch == '_'
}

/*
impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }



    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap()); // TODO: unwrap()
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LowerThan,
            Some('>') => Token::GreaterThan,
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,

            Some(ch @ _) => {
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    token::lookup_ident(&literal)
                } else if ch.is_numeric() {
                    Token::Integer(self.read_number(ch))
                } else {
                    Token::Illegal // TODO: Maybe we need ch here, to display a nice error message later?
                }
            }

            // Handle EOF
            None => Token::EndOfFile,
        }
    }
}

// is_letter checks whether a char is a valid alphabetic character or an underscore

*/

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

}
