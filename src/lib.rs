pub mod ast;
pub mod back;
mod front;
mod loc;

use back::env::SmartEnv;
use loc::Loc;

pub fn parse_eval_print(env: SmartEnv, filename: &str, input: &str) -> String {
    let parse_result = front::parse(filename, input);

    match parse_result {
        Ok(nodes) => {
            let eval_result = back::eval(env, nodes);
            match eval_result {
                Ok(output_node) => format!("{}", output_node.val),
                Err(runtime_error) => match runtime_error.loc() {
                    Loc::File { filename, line, .. } => format!(
                        "Runtime error ({}:{}): {}\n",
                        filename,
                        line,
                        runtime_error.display()
                    ),
                    Loc::Unknown => format!("Runtime error: {}\n", runtime_error.display()),
                },
            }
        }
        Err(syntax_errors) => {
            let mut output = String::new();
            for syntax_error in syntax_errors {
                let s = match syntax_error.loc() {
                    Loc::File { filename, line, .. } => format!(
                        "Syntax error ({}:{}): {}\n",
                        filename,
                        line,
                        syntax_error.display()
                    ),
                    Loc::Unknown => format!("Syntax error: {}\n", syntax_error.display(),),
                };
                output.push_str(&s);
            }
            output
        }
    }
}
