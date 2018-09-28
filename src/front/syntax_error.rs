use front::tokens::Token;
use loc::Loc;

pub type WrappedSyntaxErrors = Vec<(Loc, SyntaxError)>;

pub enum SyntaxError {
    UnparsableNumber(String),
    UnparsableCharacter(String),
    UnrecognizedToken(Token),
}

impl SyntaxError {
    pub fn display(&self) -> String {
        use self::SyntaxError::*;
        match self {
            UnparsableNumber(s) => format!("Unparsable number literal: {}", s),
            UnparsableCharacter(s) => format!("Unparsable character literal: {}", s),
            UnrecognizedToken(t) => format!("Unrecognized token: {}", t.display()),
        }
    }
}
