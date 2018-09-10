mod ast;
mod back;
mod front;
mod loc;

use loc::Loc;

pub fn interpret(filename: &str, input: &str) -> String {
    let parse_result = front::parse(filename, input);

    match parse_result {
        Ok(wrapped_nodes) => {
            let mut env = back::create_root_env();

            /* DEBUG
            for node in &nodes {
                println!("{}", node.display())
            }
            */

            let eval_result = back::eval(&mut env, wrapped_nodes);
            match eval_result {
                Ok(output_node) => output_node.display(),
                Err(runtime_error) => match runtime_error.loc() {
                    Loc::File { filename, line, .. } => format!(
                        "Runtime error ({}:{}): {}\n\n",
                        filename,
                        line,
                        runtime_error.display()
                    ),
                    Loc::Unknown => format!("Runtime error: {}\n\n", runtime_error.display(),),
                },
            }
        }
        Err(syntax_errors) => {
            let mut output = String::new();
            for (loc, syntax_error) in syntax_errors {
                let s = match loc {
                    Loc::File { filename, line, .. } => format!(
                        "Syntax error ({}:{}): {}\n\n",
                        filename,
                        line,
                        syntax_error.display()
                    ),
                    Loc::Unknown => format!("Syntax error: {}\n\n", syntax_error.display(),),
                };
                output.push_str(&s);
            }
            output
        }
    }
}
