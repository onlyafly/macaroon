use ast::*;
use loc::Loc;
use scanner;
use tokens::Token;

type SyntaxErrors = Vec<(Loc, String)>;

pub fn parse(filename: &str, input: &str) -> Result<Vec<WrappedNode>, SyntaxErrors> {
    let mut p = Parser::new(filename, input);
    let mut nodes = Vec::new();
    let mut errors = Vec::<(Loc, String)>::new();
    p.next_token();

    while p.current_token != Token::EndOfFile {
        let n = p.parse_node(&mut errors);
        nodes.push(n);
        p.next_token();
        //DEBUG println!("processing: {:?}", p.current_token);
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(nodes)
    }
}

struct Parser<'a> {
    scanner: scanner::Scanner<'a>,
    current_token: Token,
    current_loc: Loc,
}

impl<'a> Parser<'a> {
    fn new(filename: &'a str, input: &'a str) -> Parser<'a> {
        let s = scanner::Scanner::new(filename, input);
        Parser {
            scanner: s,
            current_token: Token::Error("START".to_string()),
            current_loc: Loc {
                filename: "<start>".to_string(),
                line: 0,
                pos: 0,
            },
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.scanner.next();
        self.current_loc = self.scanner.loc();
    }

    fn register_error(&self, errors: &mut SyntaxErrors, msg: &str) {
        errors.push((self.current_loc.clone(), msg.to_string()));
    }

    fn wrap(&self, n: Node) -> WrappedNode {
        WrappedNode::new(n, self.current_loc.clone())
    }

    fn parse_node(&mut self, errors: &mut SyntaxErrors) -> WrappedNode {
        let node = match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Node::Number(number),
                    Err(_) => {
                        self.register_error(errors, &format!("Unable to parse number: {}", s));

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
                self.register_error(errors, &format!("Unrecognized token: {}", t.display()));
                // Try to recover by pushing an error node
                Node::Error(t.display())
            }
        };

        self.wrap(node)
    }
}
