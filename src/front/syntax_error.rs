use front::tokens::Token;
use loc::Loc;

pub type WrappedSyntaxErrors = Vec<(Loc, SyntaxError)>;

pub enum SyntaxError {
    UnparsableNumber(String),
    UnrecognizedToken(Token),
}

impl SyntaxError {
    pub fn display(&self) -> String {
        use self::SyntaxError::*;
        match self {
            UnparsableNumber(s) => format!("Unparsable number: {}", s),
            UnrecognizedToken(t) => format!("Unrecognized token: {}", t.display()),
        }
    }
}
