use ast::Value;
use loc::Loc;

pub enum RuntimeError {
    UndefinedName(String, Loc),
    CannotRedefine(String, Loc),
    CannotUpdateUndefinedName(String, Loc),
    UnableToEvalValue(Value, Loc),
    UnableToEvalListStartingWith(String, Loc),
    UnexpectedValue(String, Value, Loc),
    CannotUpdateElementInValue(Value, Loc),
}

impl RuntimeError {
    pub fn display(&self) -> String {
        match self {
            RuntimeError::UndefinedName(name, _) => format!("Undefined name: {}", name),
            RuntimeError::CannotRedefine(name, _) => format!("Cannot redefine a name: {}", name),
            RuntimeError::CannotUpdateUndefinedName(name, _) => {
                format!("Cannot update an undefined name: {}", name)
            }
            RuntimeError::UnableToEvalValue(value, _) => {
                format!("Unable to eval value: {}", value.display())
            }
            RuntimeError::UnableToEvalListStartingWith(name, _) => {
                format!("Unable to eval list starting with: {}", name)
            }
            RuntimeError::UnexpectedValue(expected_string, got_value, _) => format!(
                "Unexpected value. Expected {} but got: {}",
                expected_string,
                got_value.display(),
            ),
            RuntimeError::CannotUpdateElementInValue(value, _) => {
                format!("Cannot update an element in: {}", value.display())
            }
        }
    }

    pub fn loc(&self) -> Loc {
        match self {
            RuntimeError::UndefinedName(_, l) => l.clone(),
            RuntimeError::CannotRedefine(_, l) => l.clone(),
            RuntimeError::CannotUpdateUndefinedName(_, l) => l.clone(),
            RuntimeError::UnableToEvalValue(_, l) => l.clone(),
            RuntimeError::UnableToEvalListStartingWith(_, l) => l.clone(),
            RuntimeError::UnexpectedValue(_, _, l) => l.clone(),
            RuntimeError::CannotUpdateElementInValue(_, l) => l.clone(),
        }
    }
}
