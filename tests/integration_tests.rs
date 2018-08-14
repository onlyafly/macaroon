// extern crate we're testing, same as any other code would do.
extern crate quivi;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
//use std::path::Path;
use std::ffi::OsStr;

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
            // Create a path to the desired file
            //let path = Path::new("testsuite/1.q");
            let display = path.display();

            println!("file: {}", display);

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

            assert_eq!("1", s.trim_right());
        }
    }
}
