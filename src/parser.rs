use ast::*;
use scanner;
use tokens::Token;

pub struct Parser<'a> {
    scanner: scanner::Scanner<'a>,
    current_token: Token,
    syntax_errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        let s = scanner::Scanner::new(input);
        Parser {
            scanner: s,
            current_token: Token::Error,
            syntax_errors: Vec::<String>::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        self.next_token();

        while self.current_token != Token::EndOfFile {
            let n = self.parse_node();
            nodes.push(n);
            self.next_token();
        }

        nodes
    }

    fn next_token(&mut self) {
        self.current_token = self.scanner.next();
    }

    fn parse_node(&mut self) -> Node {
        match self.current_token {
            Token::Number(ref s) => {
                match s.parse::<i32>() {
                    Ok(number) => Node::Number(number),
                    Err(_) => {
                        // TODO make error more valuable
                        self.syntax_errors
                            .push("Unable to parse number".to_string());
                        // Recover from error by continuing with a dummy value
                        Node::Number(0)
                    }
                }
            }
            Token::SingleQuote => {
                self.next_token();
                let quoted_node = self.parse_node();
                let child_nodes = vec![Node::Symbol("quote".to_string()), quoted_node];
                let l = Node::List(child_nodes);
                l
            }
            _ => Node::Error,
        }
    }
}
