extern crate quivi;
extern crate rustyline;

use quivi::ast::{ReaderObj, WriterObj};
use quivi::back;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::rc::Rc;

fn reader_function() -> Result<String, String> {
    use std::io;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_count_bytes_read) => {
            return Ok(input);
        }
        Err(error) => Err(format!("{}", error)),
    }
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let w = WriterObj::Standard;
    let r = ReaderObj { reader_function };
    let env = match back::create_root_env(w, r) {
        Ok(env) => env,
        Err(_) => panic!("Problem creating root environment"),
    };

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                let output = quivi::parse_eval_print(Rc::clone(&env), "REPL", &line);
                println!("{}", output);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Pressed CTRL-C... ending session");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Pressed CTRL-D... ending session");
                break;
            }
            Err(err) => {
                println!("Readline error: {:?}", err);
                break;
            }
        }
    }
    //TODO rl.save_history("history.txt").unwrap();
}
