use ast::Node;
use ast::Val;
use front::syntax_error::SyntaxError;
use loc::Loc;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    Unknown(String, Loc),
    UndefinedPrimitive(String, Loc),
    UndefinedName(String, Loc),
    CannotRedefine(String, Loc),
    CannotEvalEmptyList(Loc),
    CannotUpdateUndefinedName(String, Loc),
    UnableToEvalValue(Val, Loc),
    UnableToEvalListStartingWith(String, Loc),
    UnexpectedValue(String, Val, Loc),
    CannotUpdateElementInValue(Val, Loc),
    IndexOutOfBounds {
        index: usize,
        len: usize,
        loc: Loc,
    },
    NotEnoughArgs(String, isize, usize, Loc),
    WrongNumberOfArgs(String, isize, usize, Loc),
    ArgCountOutOfRange(String, isize, isize, usize, Loc),
    ParamsMustBeSymbols(Val, Loc),
    CondUnmatchedClause(Val, Loc),
    ApplicationPanic(String, Loc),
    CannotInvokeNonProcedure(String, Loc),
    FunctionArgsDoNotMatchParams {
        function_name: Option<String>,
        params_count: usize,
        args_count: usize,
        params_list: Vec<Node>,
        args_list: Vec<Node>,
        loc: Loc,
    },
    TooManyFunctionParamsAfterRest {
        function_name: Option<String>,
        remaining_params: Vec<Node>,
        loc: Loc,
    },
    CannotAppendOnto(Val, Loc),
    CannotGetChildrenOfNonCollection(Val, Loc),
    CannotConsOntoNonCollection(Val, Loc),
    CannotConsNonCharacterOntoString(Val, Loc),
    CannotGetLengthOfNonCollection(Val, Loc),
    SyntaxErrorDuringRead(String, SyntaxError, Loc),
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
            UnableToEvalValue(val, _) => format!("Unable to eval val: {}", val),
            UnableToEvalListStartingWith(name, _) => {
                format!("Unable to eval list starting with: {}", name)
            }
            UnexpectedValue(expected_string, got_value, _) => format!(
                "Unexpected value. Expected {} but got: {}",
                expected_string, got_value,
            ),
            CannotUpdateElementInValue(val, _) => format!("Cannot update an element in: {}", val),
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
            ParamsMustBeSymbols(v, _ ) => format!("Parameters must be symbols: {}", v),
            CondUnmatchedClause(val, _) => format!(
                "'cond' expects each clause to have a test and a body, but found: {}",
                val
            ),
            ApplicationPanic(s, _) => format!("Application Panic: {}", s),
            CannotInvokeNonProcedure(s, _) => format!("Cannot invoke a non-procedure: {}", s),
            FunctionArgsDoNotMatchParams {
                function_name,
                params_count,
                args_count,
                params_list,
                args_list,
                ..
            } => format!("Function{} expects {} argument(s), but was given {}. Function parameter list: {}. Arguments: {}",
                match function_name {
                    Some(s) => format!(" '{}'", s),
                    None => String::new(),
                },
                params_count,
                args_count,
                Val::List(params_list.to_vec()),
                Val::List(args_list.to_vec()),
            ),
            TooManyFunctionParamsAfterRest {
                function_name,
                remaining_params,
                ..
            } => format!("Function{} should have exactly one parameter after '&rest', but found {}",
                match function_name {
                    Some(s) => format!(" '{}'", s),
                    None => String::new(),
                },
                Val::List(remaining_params.to_vec()),
            ),
            CannotAppendOnto(val, _) => format!("Cannot append onto: {}", val),
            CannotGetChildrenOfNonCollection(val, _) => format!("Cannot get children of a non-collection: {}", val),
            CannotConsOntoNonCollection(val, _) => format!("Cannot cons onto a non-collection: {}", val),
            CannotConsNonCharacterOntoString(val, _) => format!("Cannot cons non-character onto a string: {}", val),
            CannotGetLengthOfNonCollection(val, _) => format!("Cannot get length of a non-collection: {}", val),
            SyntaxErrorDuringRead(s, syntax_error, _) => format!("Unable to read string \"{}\": {}", s, syntax_error.display()),
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
            ParamsMustBeSymbols(.., loc) => loc.clone(),
            CondUnmatchedClause(.., loc) => loc.clone(),
            ApplicationPanic(.., loc) => loc.clone(),
            CannotInvokeNonProcedure(.., loc) => loc.clone(),
            CannotAppendOnto(.., loc) => loc.clone(),
            CannotGetChildrenOfNonCollection(.., loc) => loc.clone(),
            FunctionArgsDoNotMatchParams { loc, .. } => loc.clone(),
            TooManyFunctionParamsAfterRest { loc, .. } => loc.clone(),
            CannotConsOntoNonCollection(.., loc) => loc.clone(),
            CannotConsNonCharacterOntoString(.., loc) => loc.clone(),
            CannotGetLengthOfNonCollection(.., loc) => loc.clone(),
            SyntaxErrorDuringRead(.., loc) => loc.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_args() {
        // Arrange
        //let args = vec![Node::new(Val::Number(42), Loc::Unknown)];
        let args = Vec::<Node>::new();

        // Act
        let r = check_args("list", &Loc::Unknown, &args, 1, -1);

        // Assert
        assert_eq!(
            r,
            Err(RuntimeError::NotEnoughArgs(
                "list".to_string(),
                1,
                0,
                Loc::Unknown
            ))
        );
    }
}
