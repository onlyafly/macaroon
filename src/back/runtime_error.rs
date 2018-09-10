use ast::Node;
use loc::Loc;

pub enum RuntimeError {
    UndefinedName(String, Loc),
    CannotRedefine(String, Loc),
    CannotUpdateUndefinedName(String, Loc),
    UnableToEvalNode(Node, Loc),
    UnableToEvalListStartingWith(String, Loc),
    UnexpectedNode(String, Node, Loc),
    CannotUpdateElementInNode(Node, Loc),
}

impl RuntimeError {
    pub fn display(&self) -> String {
        match self {
            RuntimeError::UndefinedName(name, _) => format!("Undefined name: {}", name),
            RuntimeError::CannotRedefine(name, _) => format!("Cannot redefine a name: {}", name),
            RuntimeError::CannotUpdateUndefinedName(name, _) => {
                format!("Cannot update an undefined name: {}", name)
            }
            RuntimeError::UnableToEvalNode(node, _) => {
                format!("Unable to eval node: {}", node.display())
            }
            RuntimeError::UnableToEvalListStartingWith(name, _) => {
                format!("Unable to eval list starting with: {}", name)
            }
            RuntimeError::UnexpectedNode(expected_string, got_node, _) => format!(
                "Unexpected node. Expected {} but got: {}",
                expected_string,
                got_node.display(),
            ),
            RuntimeError::CannotUpdateElementInNode(node, _) => {
                format!("Cannot update an element in: {}", node.display())
            }
        }
    }

    pub fn loc(&self) -> Loc {
        match self {
            RuntimeError::UndefinedName(_, l) => l.clone(),
            RuntimeError::CannotRedefine(_, l) => l.clone(),
            RuntimeError::CannotUpdateUndefinedName(_, l) => l.clone(),
            RuntimeError::UnableToEvalNode(_, l) => l.clone(),
            RuntimeError::UnableToEvalListStartingWith(_, l) => l.clone(),
            RuntimeError::UnexpectedNode(_, _, l) => l.clone(),
            RuntimeError::CannotUpdateElementInNode(_, l) => l.clone(),
        }
    }
}
