use ast::{Node, WrappedNode};
use eval;
use eval::env::Env;
use eval::RuntimeError;
use loc::Loc;

pub fn eval_special_list(
    env: &mut Env,
    loc: Loc,
    args: Vec<WrappedNode>,
) -> Result<WrappedNode, RuntimeError> {
    let mut evaled_args = Vec::new();

    for child in args {
        let evaled_child = eval::eval_node(env, child)?;
        evaled_args.push(evaled_child);
    }

    Ok(WrappedNode::new(
        Node::List {
            children: evaled_args,
        },
        loc,
    ))
}

pub fn eval_special_quote(mut args: Vec<WrappedNode>) -> Result<WrappedNode, RuntimeError> {
    Ok(args.remove(0))
}

pub fn eval_special_def(
    env: &mut Env,
    mut args: Vec<WrappedNode>,
) -> Result<WrappedNode, RuntimeError> {
    let name_wrapped_node = args.remove(0);

    if let Node::Symbol(name) = name_wrapped_node.node {
        if env.exists(&name) {
            return Err(RuntimeError::Simple(format!(
                "Cannot redefine a name: {}",
                name
            )));
        }

        let value_wrapped_node = super::eval_node(env, args.remove(0))?;

        env.insert(name, value_wrapped_node);
        Ok(WrappedNode::new(Node::Number(0), name_wrapped_node.loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::Simple(format!(
            "Expected symbol, got {}",
            name_wrapped_node.display()
        )))
    }
}

pub fn eval_special_update(
    env: &mut Env,
    mut args: Vec<WrappedNode>,
) -> Result<WrappedNode, RuntimeError> {
    let name_wrapped_node = args.remove(0);

    if let Node::Symbol(name) = name_wrapped_node.node {
        if !env.exists(&name) {
            return Err(RuntimeError::Simple(format!(
                "Cannot update an undefined name: {}",
                name
            )));
        }
        let value = args.remove(0);
        env.insert(name, value);
        Ok(WrappedNode::new(Node::Number(0), name_wrapped_node.loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::Simple(format!(
            "Expected symbol, got {}",
            name_wrapped_node.display()
        )))
    }
}

pub fn eval_special_update_element(
    env: &mut Env,
    mut args: Vec<WrappedNode>,
) -> Result<WrappedNode, RuntimeError> {
    let name_wrapped_node = args.remove(0);
    let node = name_wrapped_node.node;
    let loc = name_wrapped_node.loc;

    if let Node::Symbol(name) = node {
        if !env.exists(&name) {
            return Err(RuntimeError::Simple(format!(
                "Cannot update an undefined name: {}",
                name
            )));
        }

        let _index_node = args.remove(0);
        let value_node = super::eval_node(env, args.remove(0))?;

        if let Some(entry) = env.remove(&name) {
            match entry.node {
                Node::List { mut children } => {
                    //TODO: get num from index_node instead of using zero
                    children[0] = value_node;
                    env.insert(name, WrappedNode::new(Node::List { children }, loc.clone()));
                }
                _ => {
                    return Err(RuntimeError::Simple(format!(
                        "Tried to update an element in a non-list: {}",
                        entry.display()
                    )));
                }
            }
        }

        Ok(WrappedNode::new(Node::Number(0), loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::Simple(format!(
            "Expected symbol, got {}",
            node.display()
        )))
    }
}

pub fn eval_special_fn(
    _env: &mut Env,
    mut args: Vec<WrappedNode>,
) -> Result<WrappedNode, RuntimeError> {
    let param_list = args.remove(0);
    let body = args;

    match param_list.node {
        Node::List { children } => Ok(WrappedNode::new(
            Node::Proc {
                params: children,
                body: body,
            },
            param_list.loc,
        )),
        _ => Err(RuntimeError::Simple(format!(
            "Expected list of paramters, got {}",
            param_list.display()
        ))),
    }
}
