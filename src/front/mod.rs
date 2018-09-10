mod parser;
mod scanner;
mod syntax_error;
mod tokens;

use ast::*;
use front::parser::Parser;
use front::syntax_error::WrappedSyntaxErrors;
use front::tokens::Token;

pub fn parse(filename: &str, input: &str) -> Result<Vec<Node>, WrappedSyntaxErrors> {
    let mut p = Parser::new(filename, input);
    let mut values = Vec::new();
    let mut errors = WrappedSyntaxErrors::new();
    p.next_token();

    while p.current_token != Token::EndOfFile {
        let n = p.parse_value(&mut errors);
        values.push(n);
        p.next_token();
        //DEBUG println!("processing: {:?}", p.current_token);
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(values)
    }
}
