mod ast;
mod eval;
mod loc;
mod parser;
mod scanner;
mod tokens;

use eval::RuntimeError;

pub fn interpret(filename: &str, input: &str) -> String {
    let parse_result = parser::parse(filename, input);

    match parse_result {
        Ok(wrapped_nodes) => {
            let mut env = eval::create_root_env();

            /* DEBUG
            for node in &nodes {
                println!("{}", node.display())
            }
            */

            match eval::eval(&mut env, wrapped_nodes) {
                Ok(output_node) => output_node.display(),
                Err(RuntimeError::Rich(msg, loc)) => {
                    format!("Runtime error ({}:{}): {}\n\n", loc.filename, loc.line, msg)
                }
                Err(RuntimeError::Simple(msg)) => format!("Runtime error: {}\n\n", msg),
            }
        }
        Err(syntax_errors) => {
            let mut output = String::new();
            for (loc, syntax_error) in syntax_errors {
                output.push_str(&format!(
                    "Syntax error ({}:{}): {}\n\n",
                    loc.filename,
                    loc.line,
                    syntax_error.display(),
                ));
            }
            output
        }
    }
}
