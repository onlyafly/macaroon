// extern crate we're testing, same as any other code would do.
extern crate colored;
extern crate quivi;

use colored::*;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[test]
fn test_suite() {
    let mut failures = Vec::new();

    for folder_entry in fs::read_dir("./testsuite/").unwrap() {
        let unwrapped_folder_entry = folder_entry.unwrap();
        if unwrapped_folder_entry.file_type().unwrap().is_dir() {
            let folder_entry_path = unwrapped_folder_entry.path();

            for file_entry_result in fs::read_dir(folder_entry_path).unwrap() {
                let path = file_entry_result.unwrap().path();

                if Some(OsStr::new("q")) == path.extension() {
                    let input_contents = read_text_contents(&path);
                    let actual_output = quivi::interpret(input_contents.trim_right());

                    if let Some(output_file_stem) = path.file_stem() {
                        let case: String = output_file_stem.to_str().unwrap().to_owned();
                        let output_path = path.parent().unwrap().join(case.clone() + ".out");
                        let expected_output = read_text_contents(&output_path);

                        if expected_output.trim_right() != actual_output {
                            failures.push((case, expected_output, actual_output));
                        }
                    }
                }
            }
        }
    }

    if failures.len() > 0 {
        println!("\n{}\n", "Quivi Test Suite Failures".magenta().bold());

        let mut count: i32 = 1;
        for failure in failures {
            let (case, expected, actual) = failure;
            println!(
                "{}{} ({})\n\n   {}:\n\n\t{}\n\n   {}:\n\n\t{}\n\n",
                "Failure #".magenta().bold(),
                count.to_string().magenta().bold(),
                case.blue(),
                "Expected".bold(),
                expected.trim_right().green().bold(),
                "Actual".bold(),
                actual.red().bold(),
            );

            count += 1;
        }

        panic!("Test cases failed.");
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
        Ok(_) => {}
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed

    s
}
