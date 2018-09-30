use ast::*;
use front::scanner;
use front::syntax_error::{SyntaxError, WrappedSyntaxErrors};
use front::tokens::Token;
use loc::Loc;

pub struct Parser<'a> {
    scanner: scanner::Scanner<'a>,
    pub current_token: Token,
    pub current_loc: Loc,
}

impl<'a> Parser<'a> {
    pub fn new(filename: &'a str, input: &'a str) -> Parser<'a> {
        let s = scanner::Scanner::new(filename, input);
        Parser {
            scanner: s,
            current_token: Token::Error("START".to_string()),
            current_loc: Loc::File {
                filename: "<start>".to_string(),
                line: 0,
                pos: 0,
            },
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.scanner.next();
        self.current_loc = self.scanner.loc();
    }

    pub fn parse_value(&mut self, errors: &mut WrappedSyntaxErrors) -> Node {
        let value = match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Value::Number(number),
                    Err(_) => {
                        self.register_error(errors, SyntaxError::UnparsableNumber(s.to_string()));

                        // Recover from error by continuing with a dummy value
                        Value::Number(0)
                    }
                }
            }
            Token::Character { ref value, ref raw } => {
                match raw.as_ref() {
                    r"\newline" => Value::Character("\n".to_string()),
                    x if x.len() == 2 => Value::Character(value.to_string()),
                    x => {
                        self.register_error(
                            errors,
                            SyntaxError::UnparsableCharacter(x.to_string()),
                        );

                        // Recover from error by continuing with a dummy value
                        Value::Error(x.to_string())
                    }
                }
            }

            Token::StringLiteral(ref s) => Value::StringVal(s.clone()),
            Token::Symbol(ref s) => Value::Symbol(s.clone()),
            Token::SingleQuote => {
                self.next_token();
                let quoted_value = self.parse_value(errors);
                let children = vec![
                    self.make_node(Value::Symbol("quote".to_string())),
                    quoted_value,
                ];
                Value::List { children }
            }
            Token::LeftParen => {
                self.next_token();
                let mut children = Vec::<Node>::new();

                while self.current_token != Token::EndOfFile
                    && self.current_token != Token::RightParen
                {
                    children.push(self.parse_value(errors));
                    self.next_token();
                }

                Value::List { children }
            }
            ref t => {
                self.register_error(errors, SyntaxError::UnrecognizedToken(t.clone()));
                // Try to recover by pushing an error Value
                Value::Error(t.display())
            }
        };

        self.make_node(value)
    }

    fn register_error(&self, errors: &mut WrappedSyntaxErrors, e: SyntaxError) {
        errors.push((self.current_loc.clone(), e));
    }

    fn make_node(&self, n: Value) -> Node {
        Node::new(n, self.current_loc.clone())
    }
}
