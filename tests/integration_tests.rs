// extern crate we're testing, same as any other code would do.
extern crate quivi;

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

#[test]
fn test_add() {
    assert_eq!(quivi::add(3, 2), 5);
}

#[test]
fn test_suite() {
    let paths = fs::read_dir("./testsuite/").unwrap();

    for wrapped_path in paths {
        let path = wrapped_path.unwrap().path();

        if Some(OsStr::new("q")) == path.extension() {
            let input_contents = read_text_contents(&path);

            //TODO: calculate input_result correctly
            let actual_output = quivi::interpret(input_contents.trim_right());

            if let Some(output_file_stem) = path.file_stem() {
                let output_path = Path::new("./testsuite")
                    .join(output_file_stem.to_str().unwrap().to_owned() + ".out");
                let expected_output = read_text_contents(&output_path);

                assert_eq!(expected_output.trim_right(), actual_output);
            }
        }
    }
}

fn read_text_contents(path: &PathBuf) -> String {
    // Create a path to the desired file
    //let path = Path::new("testsuite/1.q");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed

    s
}
