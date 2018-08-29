mod env;
mod eval;
mod nodes;
mod parser;
mod scanner;
mod tokens;

use env::Env;

pub fn interpret(input: &str) -> String {
    let parse_result = parser::parse(input);

    match parse_result {
        Ok(nodes) => {
            let mut env = Env::new();

            /* DEBUG
            for node in &nodes {
                println!("{}", node.display())
            }
            */

            match eval::eval(&mut env, nodes) {
                Ok(output_node) => output_node.display(),
                Err(message) => message,
            }
        }
        Err(mut syntax_errors) => {
            syntax_errors.remove(0) //TODO
        }
    }
}
