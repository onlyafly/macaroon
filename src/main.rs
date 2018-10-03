extern crate quivi;
extern crate rustyline;

use quivi::ast::{ReaderObj, WriterObj};
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                let output = quivi::interpret(
                    "REPL",
                    &line,
                    WriterObj::Standard,
                    ReaderObj { reader_function },
                );
                println!("{}", output);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    //TODO rl.save_history("history.txt").unwrap();
}
