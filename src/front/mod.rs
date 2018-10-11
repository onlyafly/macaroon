mod parser;
mod scanner;
pub mod syntax_error;
mod tokens;

use ast::*;
use front::parser::Parser;
use front::syntax_error::SyntaxError;
use front::tokens::Token;

pub fn parse(filename: &str, input: &str) -> Result<Vec<Node>, Vec<SyntaxError>> {
    let mut p = Parser::new(filename, input);
    let mut values = Vec::new();
    let mut errors = Vec::new();
    p.next_token(&mut errors);

    while p.current_token != Token::EndOfFile {
        let n = p.parse_value(&mut errors);
        values.push(n);
        p.next_token(&mut errors);
        //DEBUG println!("processing: {:?}", p.current_token);
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(values)
    }
}
