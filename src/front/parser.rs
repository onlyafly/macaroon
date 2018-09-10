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

    pub fn parse_node(&mut self, errors: &mut WrappedSyntaxErrors) -> WrappedNode {
        let node = match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Node::Number(number),
                    Err(_) => {
                        self.register_error(errors, SyntaxError::UnparsableNumber(s.to_string()));

                        // Recover from error by continuing with a dummy value
                        Node::Number(0)
                    }
                }
            }

            Token::Symbol(ref s) => Node::Symbol(s.clone()),
            Token::SingleQuote => {
                self.next_token();
                let quoted_node = self.parse_node(errors);
                let children = vec![self.wrap(Node::Symbol("quote".to_string())), quoted_node];
                Node::List { children }
            }
            Token::LeftParen => {
                self.next_token();
                let mut children = Vec::<WrappedNode>::new();

                while self.current_token != Token::EndOfFile
                    && self.current_token != Token::RightParen
                {
                    children.push(self.parse_node(errors));
                    self.next_token();
                }

                Node::List { children }
            }
            ref t => {
                self.register_error(errors, SyntaxError::UnrecognizedToken(t.clone()));
                // Try to recover by pushing an error node
                Node::Error(t.display())
            }
        };

        self.wrap(node)
    }

    fn register_error(&self, errors: &mut WrappedSyntaxErrors, e: SyntaxError) {
        errors.push((self.current_loc.clone(), e));
    }

    fn wrap(&self, n: Node) -> WrappedNode {
        WrappedNode::new(n, self.current_loc.clone())
    }
}
