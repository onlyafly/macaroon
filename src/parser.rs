use ast::*;
use scanner;
use tokens::Token;

pub struct Parser<'a> {
    scanner: scanner::Scanner<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        let s = scanner::Scanner::new(input);
        Parser {
            scanner: s,
            current_token: Token::Error,
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
            Token::Number(_) => Node::Number(1234), //TODO
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
