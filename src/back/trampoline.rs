use ast::Node;
use back::env::SmartEnv;
use back::runtime_error::RuntimeError;

pub type Thunk = fn(SmartEnv, Node) -> ContinuationResult;

pub type ContinuationResult = Result<Continuation, RuntimeError>;

// If it contains a Next, the thunk is the next computation to execute.
// If it contains a Node, the trampolining session is over and the Node represents the result.
pub enum Continuation {
    Next(Thunk, SmartEnv, Node),
    Response(Node),
}

// Trampoline iteratively calls a chain of thunks until there is no next thunk,
// at which point it pulls the resulting ast.Node out of the packet and returns it.
pub fn start(t: Thunk, e: SmartEnv, n: Node) -> Result<Node, RuntimeError> {
    let mut current_t = t;
    let mut current_e = e;
    let mut current_n = n;
    loop {
        let k = current_t(current_e, current_n)?;
        match k {
            Continuation::Next(next_t, next_e, next_n) => {
                current_t = next_t;
                current_e = next_e;
                current_n = next_n;
            }
            Continuation::Response(n) => return Ok(n),
        }
    }
}
