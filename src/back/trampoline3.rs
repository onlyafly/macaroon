use ast::Node;
use back::runtime_error::RuntimeError;

pub type ContinuationResult<F> = Result<Continuation<F>, RuntimeError>;

// If it contains a Next, the thunk is the next computation to execute.
// If it contains a Node, the trampolining session is over and the Node represents the result.
pub enum Continuation<F>
where
    F: Fn() -> ContinuationResult<F>,
{
    Next(F),
    Response(Node),
}

// Trampoline iteratively calls a chain of thunks until there is no next thunk,
// at which point it pulls the resulting ast.Node out of the packet and returns it.
pub fn start<F>(start_thunk: F) -> Result<Node, RuntimeError>
where
    F: Fn() -> ContinuationResult<F>,
{
    let mut current_thunk = start_thunk;
    loop {
        let output_continuation = current_thunk()?;
        match output_continuation {
            Continuation::Next(next_thunk) => current_thunk = next_thunk,
            Continuation::Response(n) => return Ok(n),
        }
    }
}
