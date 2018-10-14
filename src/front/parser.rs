use ast::*;
use front::scanner;
use front::syntax_error::SyntaxError;
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
            current_token: Token::Error,
            current_loc: Loc::File {
                filename: "<start>".to_string(),
                line: 0,
                pos: 0,
            },
        }
    }

    pub fn next_token(&mut self, errors: &mut Vec<SyntaxError>) {
        // ORDERING: Location must be updated after the next token is scanned
        self.current_token = match self.scanner.next() {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                Token::Error
            }
        };
        self.current_loc = self.scanner.loc();
    }

    pub fn parse_value(&mut self, errors: &mut Vec<SyntaxError>) -> Node {
        let val = match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Val::Number(number),
                    Err(_) => {
                        errors.push(SyntaxError::UnparsableNumberLiteral(
                            s.to_string(),
                            self.loc(),
                        ));

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
                        errors.push(SyntaxError::UnparsableCharacterLiteral(
                            x.to_string(),
                            self.loc(),
                        ));

                        // Recover from error by continuing with a dummy val
                        Val::Error(x.to_string())
                    }
                }
            }

            Token::StringLiteral(ref s) => Val::StringVal(s.clone()),
            Token::Symbol(ref s) if s == "true" => Val::Boolean(true),
            Token::Symbol(ref s) if s == "false" => Val::Boolean(false),
            Token::Symbol(ref s) if s == "nil" => Val::Nil,
            Token::Symbol(ref s) => Val::Symbol(s.clone()),
            Token::SingleQuote => {
                self.next_token(errors);
                let quoted_value = self.parse_value(errors);
                let children = vec![
                    self.make_node(Val::Symbol("quote".to_string())),
                    quoted_value,
                ];
                Val::List(children)
            }
            Token::LeftParen => {
                self.next_token(errors);
                let mut children = Vec::<Node>::new();

                while self.current_token != Token::RightParen {
                    if self.current_token == Token::EndOfFile {
                        errors.push(SyntaxError::UnbalancedParens(self.loc()));
                        return self.make_node(Val::Error(String::new())); // Try to recover by pushing an error Val
                    }

                    children.push(self.parse_value(errors));
                    self.next_token(errors);
                }

                Val::List(children)
            }
            Token::RightParen => {
                errors.push(SyntaxError::UnbalancedParens(self.loc()));
                Val::Error(")".to_string()) // Try to recover by pushing an error Val
            }
            Token::Error => Val::Error(String::new()),
            ref t => {
                errors.push(SyntaxError::UnrecognizedToken(t.clone(), self.loc()));
                Val::Error(t.display()) // Try to recover by pushing an error Val
            }
        };

        self.make_node(val)
    }

    fn loc(&self) -> Loc {
        self.current_loc.clone()
    }

    fn make_node(&self, n: Val) -> Node {
        Node::new(n, self.current_loc.clone())
    }
}
