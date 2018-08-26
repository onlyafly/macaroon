pub struct Scanner {
    input: String,
}

impl Scanner {
    pub fn new(input: &str) -> Scanner {
        Scanner {
            input: input.to_string(),
        }
    }
}
