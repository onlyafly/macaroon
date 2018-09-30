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
        let val = match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Val::Number(number),
                    Err(_) => {
                        self.register_error(errors, SyntaxError::UnparsableNumber(s.to_string()));

                        // Recover from error by continuing with a dummy val
                        Val::Number(0)
                    }
                }
            }
            Token::Character { ref val, ref raw } => {
                match raw.as_ref() {
                    r"\newline" => Val::Character("\n".to_string()),
                    x if x.len() == 2 => Val::Character(val.to_string()),
                    x => {
                        self.register_error(
                            errors,
                            SyntaxError::UnparsableCharacter(x.to_string()),
                        );

                        // Recover from error by continuing with a dummy val
                        Val::Error(x.to_string())
                    }
                }
            }

            Token::StringLiteral(ref s) => Val::StringVal(s.clone()),
            Token::Symbol(ref s) => Val::Symbol(s.clone()),
            Token::SingleQuote => {
                self.next_token();
                let quoted_value = self.parse_value(errors);
                let children = vec![
                    self.make_node(Val::Symbol("quote".to_string())),
                    quoted_value,
                ];
                Val::List { children }
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

                Val::List { children }
            }
            Token::Error(ref s) => {
                self.register_error(errors, SyntaxError::ScanningError(s.to_string()));
                // Try to recover by pushing an error Val
                Val::Error(s.to_string())
            }
            ref t => {
                self.register_error(errors, SyntaxError::UnrecognizedToken(t.clone()));
                // Try to recover by pushing an error Val
                Val::Error(t.display())
            }
        };

        self.make_node(val)
    }

    fn register_error(&self, errors: &mut WrappedSyntaxErrors, e: SyntaxError) {
        errors.push((self.current_loc.clone(), e));
    }

    fn make_node(&self, n: Val) -> Node {
        Node::new(n, self.current_loc.clone())
    }
}
