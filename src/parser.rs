use nodes::*;
use scanner;
use tokens::Token;

pub fn parse(input: &str) -> Result<Vec<Node>, Vec<String>> {
    let mut p = Parser::new(input);
    let mut nodes = Vec::new();
    p.next_token();

    while p.current_token != Token::EndOfFile {
        let n = p.parse_node();
        nodes.push(n);
        p.next_token();
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
    syntax_errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(input: &str) -> Parser {
        let s = scanner::Scanner::new(input);
        Parser {
            scanner: s,
            current_token: Token::Error,
            syntax_errors: Vec::<String>::new(),
        }
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
                // Skip over the right paren
                self.next_token();

                Node::List(children)
            }
            ref t => {
                self.syntax_errors
                    .push(format!("Unrecognized token: {:?}", t));
                Node::Error
            }
        }
    }
}
