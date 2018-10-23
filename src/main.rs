extern crate clap;
extern crate macaroon;
extern crate rustyline;

use clap::{App, Arg};
use macaroon::ast::{ReaderObj, WriterObj};
use macaroon::back;
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
    let matches = App::new("macaroon")
        .version("0.1.0")
        .about("Macaroon Interpreter")
        .author("Kevin Albrecht <onlyafly@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("*.mn file to interpret")
                .required(false)
                .index(1),
        ).arg(
            Arg::with_name("x")
                .short("x")
                .multiple(false)
                .help("Executes a script without entering the REPL"),
        ).get_matches();

    let history_path = ".macaroon_history";

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(history_path).is_err() {
        println!("No previous history.");
    }

    let w = WriterObj::Standard;
    let r = ReaderObj { reader_function };
    let env = match back::create_root_env(w, r) {
        Ok(env) => env,
        Err(_) => panic!("Problem creating root environment"),
    };

    if let Some(input_file) = matches.value_of("INPUT") {
        println!("Loading file: {}", input_file);
        let output = macaroon::parse_eval_print(
            Rc::clone(&env),
            "REPL",
            &format!("(load \"{}\")", input_file),
        );
        println!("{}", output);
    }

    // If the -x flag is set, executes a script without entering the REPL
    if matches.occurrences_of("x") == 1 {
        return;
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                let output = macaroon::parse_eval_print(Rc::clone(&env), "REPL", &line);
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

    rl.save_history(history_path).unwrap();
}
