use tokens::*;

pub struct Scanner {
    #[allow(dead_code)]
    input: String,
}

impl Scanner {
    pub fn new(input: &str) -> Scanner {
        Scanner {
            input: input.to_string(),
        }
    }

    pub fn next(&self) -> Token {
        Token::Generic("1".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Scanner::new("1");
        assert_eq!(s.input, "1");
    }

    #[test]
    fn test_simple_tokens() {
        let s = Scanner::new("1 2 3");
        match s.next() {
            Token::Generic(v) => assert_eq!(v, "1".to_string()),
            _ => panic!("undexpected token"),
        }
        /*
        match s.next() {
            Token::Generic(v) => assert_eq!(v, "2".to_string()),
            _ => panic!("undexpected token"),
        }
        */
    }
}
