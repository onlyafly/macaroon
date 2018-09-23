use ast::Node;
use ast::Value;
use loc::Loc;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    Unknown(String, Loc),
    UndefinedPrimitive(String, Loc),
    UndefinedName(String, Loc),
    CannotRedefine(String, Loc),
    CannotEvalEmptyList(Loc),
    CannotUpdateUndefinedName(String, Loc),
    UnableToEvalValue(Value, Loc),
    UnableToEvalListStartingWith(String, Loc),
    UnexpectedValue(String, Value, Loc),
    CannotUpdateElementInValue(Value, Loc),
    IndexOutOfBounds { index: usize, len: usize, loc: Loc },
    NotEnoughArgs(String, isize, usize, Loc),
    WrongNumberOfArgs(String, isize, usize, Loc),
    ArgCountOutOfRange(String, isize, isize, usize, Loc),
    ProcArgsDoNotMatchParams(String, Loc),
}

impl RuntimeError {
    pub fn display(&self) -> String {
        use self::RuntimeError::*;
        match self {
            Unknown(name, _) => format!("Unknown error: {}", name),
            UndefinedPrimitive(name, _) => format!("Undefined primitive: {}", name),
            UndefinedName(name, _) => format!("Undefined name: {}", name),
            CannotRedefine(name, _) => format!("Cannot redefine a name: {}", name),
            CannotEvalEmptyList(_) => format!("Cannot evaluate an empty list"),
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
            ProcArgsDoNotMatchParams(_, _) => {
                format!("Arguments passed to procedure do not match parameter list")
            }
        }
    }

    pub fn loc(&self) -> Loc {
        use self::RuntimeError::*;
        match self {
            Unknown(_, l) => l.clone(),
            UndefinedPrimitive(_, l) => l.clone(),
            UndefinedName(_, l) => l.clone(),
            CannotRedefine(_, l) => l.clone(),
            CannotUpdateUndefinedName(_, l) => l.clone(),
            CannotEvalEmptyList(l) => l.clone(),
            UnableToEvalValue(_, l) => l.clone(),
            UnableToEvalListStartingWith(_, l) => l.clone(),
            UnexpectedValue(_, _, l) => l.clone(),
            CannotUpdateElementInValue(_, l) => l.clone(),
            IndexOutOfBounds { loc, .. } => loc.clone(),
            NotEnoughArgs(.., loc) => loc.clone(),
            WrongNumberOfArgs(.., loc) => loc.clone(),
            ArgCountOutOfRange(.., loc) => loc.clone(),
            ProcArgsDoNotMatchParams(.., loc) => loc.clone(),
        }
    }
}

pub fn check_args(
    name: &str,
    loc: &Loc,
    args: &Vec<Node>,
    min_params: isize,
    max_params: isize,
) -> Result<(), RuntimeError> {
    if max_params == -1 {
        if (args.len() as isize) < min_params {
            return Err(RuntimeError::NotEnoughArgs(
                name.to_string(),
                min_params,
                args.len(),
                loc.clone(),
            ));
        }
    } else if (min_params == max_params) && (min_params != args.len() as isize) {
        return Err(RuntimeError::WrongNumberOfArgs(
            name.to_string(),
            min_params,
            args.len(),
            loc.clone(),
        ));
    } else if ((args.len() as isize) < min_params) || ((args.len() as isize) > max_params) {
        return Err(RuntimeError::ArgCountOutOfRange(
            name.to_string(),
            min_params,
            max_params,
            args.len(),
            loc.clone(),
        ));
    }

    Ok(())
}
