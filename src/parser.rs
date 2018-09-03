use nodes::*;
use scanner;
use tokens::Loc;
use tokens::Token;

pub fn parse(filename: &str, input: &str) -> Result<Vec<Node>, Vec<(Loc, String)>> {
    let mut p = Parser::new(filename, input);
    let mut nodes = Vec::new();
    p.next_token();

    while p.current_token != Token::EndOfFile {
        let n = p.parse_node();
        nodes.push(n);
        p.next_token();
        //DEBUG println!("processing: {:?}", p.current_token);
    }

    if p.syntax_errors.len() > 0 {
        Err(p.syntax_errors)
    } else {
        Ok(nodes)
    }
}

struct Parser<'a> {
    scanner: scanner::Scanner<'a>,
    current_token: Token,
    current_loc: Loc,
    syntax_errors: Vec<(Loc, String)>,
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
            syntax_errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.scanner.next();
        self.current_loc = self.scanner.loc();
    }

    /* FIXME
    fn register_error(&mut self, msg: &str) {
        self.syntax_errors
            .push((self.current_loc.clone(), msg.to_string()));
    }
    */

    fn parse_node(&mut self) -> Node {
        match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Node::Number(number),
                    Err(_) => {
                        // TODO make error more valuable

                        self.syntax_errors.push((
                            self.current_loc.clone(),
                            format!("Unable to parse number: {}", s),
                        ));

                        //FIXMEself.register_error(&format!("Unable to parse number: {}", s));

                        // Recover from error by continuing with a dummy value
                        Node::Number(0)
                    }
                }
            }

            Token::Symbol(ref s) => Node::Symbol(s.clone()),
            Token::SingleQuote => {
                self.next_token();
                let quoted_node = self.parse_node();
                let child_nodes = vec![Node::Symbol("quote".to_string()), quoted_node];
                let l = Node::List(child_nodes);
                l
            }
            Token::LeftParen => {
                self.next_token();
                let mut children = Vec::<Node>::new();

                while self.current_token != Token::EndOfFile
                    && self.current_token != Token::RightParen
                {
                    children.push(self.parse_node());
                    self.next_token();
                }

                Node::List(children)
            }
            ref t => {
                self.syntax_errors.push((
                    self.current_loc.clone(),
                    format!("Unrecognized token: {}", t.display()),
                ));
                // Try to recover by pushing an error node
                Node::Error(t.display())
            }
        }
    }
}
