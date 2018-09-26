use ast::Node;
use back::runtime_error::RuntimeError;

pub type Thunk = Fn() -> ContinuationResult;

pub type ContinuationResult = Result<Continuation, RuntimeError>;

// If it contains a Next, the thunk is the next computation to execute.
// If it contains a Node, the trampolining session is over and the Node represents the result.
pub enum Continuation {
    Next(Thunk),
    Response(Node),
}

// Trampoline iteratively calls a chain of thunks until there is no next thunk,
// at which point it pulls the resulting ast.Node out of the packet and returns it.
pub fn start(t: Thunk) -> Result<Node, RuntimeError> {
    let mut current_t = t;
    loop {
        let output_continuation = current_t()?;
        match output_continuation {
            Continuation::Next(next_t) => current_t = next_t,
            Continuation::Response(n) => return Ok(n),
        }
    }
}
