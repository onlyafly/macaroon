mod env;
mod eval;
mod nodes;
mod parser;
mod scanner;
mod tokens;

use env::Env;

pub fn interpret(filename: &str, input: &str) -> String {
    let parse_result = parser::parse(filename, input);

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
        Err(syntax_errors) => {
            let mut output = String::new();
            for (loc, msg) in syntax_errors {
                output.push_str(&format!(
                    "Syntax error ({}:{}): {}\n\n",
                    loc.filename, loc.line, msg
                ));
            }
            output
        }
    }
}
