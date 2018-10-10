use ast::Node;
use back::env::SmartEnv;
use back::runtime_error::RuntimeError;

pub enum Flag {
    None,
    DelayMacroEvaluation,
}

pub type Thunk = fn(SmartEnv, Node, Vec<Node>, Flag) -> ContinuationResult;

pub type ContinuationResult = Result<Continuation, RuntimeError>;

// If it contains a Next, the thunk is the next computation to execute.
// If it contains a Node, the trampolining session is over and the Node represents the result.
pub enum Continuation {
    Next(Thunk, SmartEnv, Node, Vec<Node>, Flag),
    Outcome(Node),
}

pub fn bounce(t: Thunk, e: SmartEnv, n: Node) -> Continuation {
    Continuation::Next(t, e, n, Vec::new(), Flag::None)
}

pub fn bounce_with_nodes(
    t: Thunk,
    e: SmartEnv,
    n: Node,
    ns: Vec<Node>,
    flag: Flag,
) -> Continuation {
    Continuation::Next(t, e, n, ns, flag)
}

pub fn finish(n: Node) -> Continuation {
    Continuation::Outcome(n)
}

pub fn run(t: Thunk, e: SmartEnv, n: Node) -> Result<Node, RuntimeError> {
    run_with_everything(t, e, n, Vec::new(), Flag::None)
}

pub fn run_with_nodes(t: Thunk, e: SmartEnv, n: Node, ns: Vec<Node>) -> Result<Node, RuntimeError> {
    run_with_everything(t, e, n, ns, Flag::None)
}

pub fn run_with_flag(t: Thunk, e: SmartEnv, n: Node, flag: Flag) -> Result<Node, RuntimeError> {
    run_with_everything(t, e, n, Vec::new(), flag)
}

// The trampoline iteratively calls a chain of thunks until there is no next thunk,
// at which point it pulls the resulting Node out of the continuation and returns it.
fn run_with_everything(
    t: Thunk,
    e: SmartEnv,
    n: Node,
    ns: Vec<Node>,
    flag: Flag,
) -> Result<Node, RuntimeError> {
    let mut current_t = t;
    let mut current_e = e;
    let mut current_n = n;
    let mut current_ns = ns;
    let mut current_flag = flag;
    loop {
        let k = current_t(current_e, current_n, current_ns, current_flag)?;
        match k {
            Continuation::Next(next_t, next_e, next_n, next_ns, next_flag) => {
                current_t = next_t;
                current_e = next_e;
                current_n = next_n;
                current_ns = next_ns;
                current_flag = next_flag;
            }
            Continuation::Outcome(n) => return Ok(n),
        }
    }
}
