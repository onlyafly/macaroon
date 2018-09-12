use ast::Value;
use loc::Loc;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    UndefinedName(String, Loc),
    CannotRedefine(String, Loc),
    CannotUpdateUndefinedName(String, Loc),
    UnableToEvalValue(Value, Loc),
    UnableToEvalListStartingWith(String, Loc),
    UnexpectedValue(String, Value, Loc),
    CannotUpdateElementInValue(Value, Loc),
    IndexOutOfBounds { index: usize, len: usize, loc: Loc },
    NotEnoughArgs(String, isize, usize, Loc),
    WrongNumberOfArgs(String, isize, usize, Loc),
    ArgCountOutOfRange(String, isize, isize, usize, Loc),
}

impl RuntimeError {
    pub fn display(&self) -> String {
        use self::RuntimeError::*;
        match self {
            UndefinedName(name, _) => format!("Undefined name: {}", name),
            CannotRedefine(name, _) => format!("Cannot redefine a name: {}", name),
            CannotUpdateUndefinedName(name, _) => {
                format!("Cannot update an undefined name: {}", name)
            }
            UnableToEvalValue(value, _) => format!("Unable to eval value: {}", value.display()),
            UnableToEvalListStartingWith(name, _) => {
                format!("Unable to eval list starting with: {}", name)
            }
            UnexpectedValue(expected_string, got_value, _) => format!(
                "Unexpected value. Expected {} but got: {}",
                expected_string,
                got_value.display(),
            ),
            CannotUpdateElementInValue(value, _) => {
                format!("Cannot update an element in: {}", value.display())
            }
            IndexOutOfBounds { index, len, .. } => {
                format!("Index of {} is out of bounds of length {}", index, len)
            }
            NotEnoughArgs(name, min, actual, _) => format!(
                "'{}' expects at least {} arg(s), but got {}",
                name, min, actual
            ),
            WrongNumberOfArgs(name, expected, actual, _) => {
                format!("'{}' expects {} arg(s), but got {}", name, expected, actual)
            }
            ArgCountOutOfRange(name, min, max, actual, _) => format!(
                "'{}' expects between {} and {} arg(s), but got {}",
                name, min, max, actual
            ),
        }
    }

    pub fn loc(&self) -> Loc {
        use self::RuntimeError::*;
        match self {
            UndefinedName(_, l) => l.clone(),
            CannotRedefine(_, l) => l.clone(),
            CannotUpdateUndefinedName(_, l) => l.clone(),
            UnableToEvalValue(_, l) => l.clone(),
            UnableToEvalListStartingWith(_, l) => l.clone(),
            UnexpectedValue(_, _, l) => l.clone(),
            CannotUpdateElementInValue(_, l) => l.clone(),
            IndexOutOfBounds { loc, .. } => loc.clone(),
            NotEnoughArgs(.., loc) => loc.clone(),
            WrongNumberOfArgs(.., loc) => loc.clone(),
            ArgCountOutOfRange(.., loc) => loc.clone(),
        }
    }
}
