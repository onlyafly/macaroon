use front::tokens::Token;
use loc::Loc;

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    UnparsableNumberLiteral(String, Loc),
    UnparsableCharacterLiteral(String, Loc),
    UnrecognizedCharacterSequence(String, Loc),
    UnrecognizedToken(Token, Loc),
    UnrecognizedCharacterInInput(char, Loc),
    UnterminatedMultilineComment(Loc),
    UnterminatedStringLiteral(Loc),
}

impl SyntaxError {
    pub fn display(&self) -> String {
        use self::SyntaxError::*;
        match self {
            UnparsableNumberLiteral(s, ..) => format!("Unparsable number literal: {}", s),
            UnparsableCharacterLiteral(s, ..) => format!("Unparsable character literal: {}", s),
            UnrecognizedCharacterSequence(s, ..) => {
                format!("Unrecognized character sequence: {}", s)
            }
            UnrecognizedToken(t, ..) => format!("Unrecognized token: {}", t.display()),
            UnrecognizedCharacterInInput(ch, ..) => {
                format!("Unrecognized character in input: {}", ch)
            }
            UnterminatedMultilineComment(..) => "Unterminated multiline comment".to_string(),
            UnterminatedStringLiteral(..) => "Unterminated string literal".to_string(),
        }
    }

    pub fn loc(&self) -> Loc {
        use self::SyntaxError::*;
        match self {
            UnparsableNumberLiteral(_, l) => l.clone(),
            UnparsableCharacterLiteral(_, l) => l.clone(),
            UnrecognizedCharacterSequence(_, l) => l.clone(),
            UnrecognizedToken(_, l) => l.clone(),
            UnrecognizedCharacterInInput(_, l) => l.clone(),
            UnterminatedMultilineComment(l) => l.clone(),
            UnterminatedStringLiteral(l) => l.clone(),
        }
    }
}
