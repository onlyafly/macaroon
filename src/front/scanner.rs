use front::syntax_error::SyntaxError;
use front::tokens::Token;
use loc::Loc;
use std::iter::Peekable;
use std::str::Chars;

pub type ScanResult = Result<Token, SyntaxError>;

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
    filename: &'a str,
    line: i32,
    pos: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(filename: &'a str, input: &'a str) -> Scanner<'a> {
        Scanner {
            input: input.chars().peekable(),
            filename: filename,
            line: 1,
            pos: 0,
        }
    }

    pub fn loc(&self) -> Loc {
        Loc::File {
            filename: self.filename.to_string(),
            line: self.line,
            pos: self.pos,
        }
    }

    pub fn next(&mut self) -> ScanResult {
        self.skip_whitespace();

        match self.read_char() {
            Some(';') => self.scan_single_line_comment(),
            Some('#') => {
                if let Some(&ch) = self.peek_char() {
                    if ch == '|' {
                        self.scan_multiline_comment()
                    } else {
                        Err(SyntaxError::UnrecognizedCharacterSequence(
                            format!("#{}", ch),
                            self.loc(),
                        ))
                    }
                } else {
                    Err(SyntaxError::UnrecognizedCharacterSequence(
                        "#".to_string(),
                        self.loc(),
                    ))
                }
            }
            Some('(') => Ok(Token::LeftParen),
            Some(')') => Ok(Token::RightParen),
            Some('-') => {
                if let Some(&ch) = self.peek_char() {
                    if ch.is_numeric() {
                        self.scan_number('-')
                    } else {
                        self.scan_symbol('-')
                    }
                } else {
                    Err(SyntaxError::UnrecognizedCharacterSequence(
                        "-".to_string(),
                        self.loc(),
                    ))
                }
            }
            Some('^') => Ok(Token::Caret),
            Some('\'') => Ok(Token::SingleQuote),
            Some('\\') => self.scan_character_literal(),
            Some('"') => self.scan_string_literal(),

            // TODO
            //  - Floating point numbers
            Some(ch) => {
                if ch.is_numeric() {
                    self.scan_number(ch)
                } else if is_symbolic(ch) {
                    self.scan_symbol(ch)
                } else {
                    Err(SyntaxError::UnrecognizedCharacterInInput(ch, self.loc()))
                }
            }

            None => Ok(Token::EndOfFile),
        }
    }

    fn scan_single_line_comment(&mut self) -> ScanResult {
        while let Some(&c) = self.peek_char() {
            if c == '\n' || c == '\r' {
                break;
            }
            self.read_char();
        }
        self.next()
    }

    fn scan_multiline_comment(&mut self) -> ScanResult {
        self.read_char(); // Skip '|'

        while let Some(ch) = self.read_char() {
            if ch == '|' {
                if let Some(&chnext) = self.peek_char() {
                    if chnext == '#' {
                        self.read_char(); // Consume '#'
                        return self.next();
                    }
                }
            }
        }

        Err(SyntaxError::UnterminatedMultilineComment(self.loc()))
    }

    fn scan_string_literal(&mut self) -> ScanResult {
        let mut buffer = String::new();

        loop {
            match self.read_char() {
                Some('"') => return Ok(Token::StringLiteral(buffer)),
                Some(c) => buffer.push(c),
                None => break,
            }
        }

        Err(SyntaxError::UnterminatedStringLiteral(self.loc()))
    }

    fn scan_character_literal(&mut self) -> ScanResult {
        let mut buffer = String::new();

        // First char
        if let Some(c) = self.read_char() {
            buffer.push(c);
        }

        // Additional chars
        while let Some(&c) = self.peek_char() {
            if !c.is_alphabetic() {
                break;
            }
            buffer.push(self.read_char().unwrap());
        }

        if buffer.len() > 0 {
            Ok(Token::Character {
                val: buffer.to_string(),
                raw: format!("\\{}", buffer),
            })
        } else {
            Err(SyntaxError::UnparsableCharacterLiteral(
                format!("\\{}", buffer),
                self.loc(),
            ))
        }
    }

    fn scan_number(&mut self, first: char) -> ScanResult {
        let mut number = String::new();

        number.push(first);

        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        Ok(Token::Number(number))
    }

    fn scan_symbol(&mut self, first: char) -> ScanResult {
        let mut symbol_text = String::new();
        symbol_text.push(first);

        while self.peek_is_symbolic() {
            symbol_text.push(self.read_char().unwrap());
        }

        Ok(Token::Symbol(symbol_text))
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

            match c {
                '\n' => self.line += 1,
                '\r' => self.line += 1,
                _ => {}
            }

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
        let mut s = Scanner::new("", "1 2 3");

        assert_eq!(s.next(), Ok(Token::Number("1".to_string())));
        assert_eq!(s.next(), Ok(Token::Number("2".to_string())));
        assert_eq!(s.next(), Ok(Token::Number("3".to_string())));
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }

    #[test]
    fn test_parens_and_numbers() {
        let mut s = Scanner::new("", "((1))");

        assert_eq!(s.next(), Ok(Token::LeftParen));
        assert_eq!(s.next(), Ok(Token::LeftParen));
        assert_eq!(s.next(), Ok(Token::Number("1".to_string())));
        assert_eq!(s.next(), Ok(Token::RightParen));
        assert_eq!(s.next(), Ok(Token::RightParen));
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }

    #[test]
    fn test_symbols_and_numbers_with_minus_sign() {
        let mut s = Scanner::new("", "- -aa -123");
        assert_eq!(s.next(), Ok(Token::Symbol("-".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("-aa".to_string())));
        assert_eq!(s.next(), Ok(Token::Number("-123".to_string())));
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }

    #[test]
    fn test_symbols() {
        let mut s = Scanner::new("", "a b123 cAcZ ? + - * / = < > ! & _ . <aA1+-*/=<>!&_");
        assert_eq!(s.next(), Ok(Token::Symbol("a".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("b123".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("cAcZ".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("?".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("+".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("-".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("*".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("/".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("=".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("<".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol(">".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("!".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("&".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("_".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol(".".to_string())));
        assert_eq!(s.next(), Ok(Token::Symbol("<aA1+-*/=<>!&_".to_string())));
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }

    #[test]
    fn test_miscellaneous() {
        let mut s = Scanner::new("", r"^ '");
        assert_eq!(s.next(), Ok(Token::Caret));
        assert_eq!(s.next(), Ok(Token::SingleQuote));
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }

    #[test]
    fn test_chars() {
        let mut s = Scanner::new("", r"\a");
        assert_eq!(
            s.next(),
            Ok(Token::Character {
                raw: r"\a".to_string(),
                val: "a".to_string()
            })
        );
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }

    #[test]
    fn test_errors() {
        let mut s = Scanner::new("", r"\");
        assert_eq!(
            s.next(),
            Err(SyntaxError::UnparsableCharacterLiteral(
                "\\".to_string(),
                Loc::File {
                    filename: "".to_string(),
                    line: 1,
                    pos: 0
                }
            ))
        );
    }

    #[test]
    fn test_quoting() {
        let mut s = Scanner::new("", r"'a");
        assert_eq!(s.next(), Ok(Token::SingleQuote));
        assert_eq!(s.next(), Ok(Token::Symbol("a".to_string())));
        assert_eq!(s.next(), Ok(Token::EndOfFile));
    }
}
