use ast::{Node, WrappedNode};
use back::env::Env;
use back::eval;
use back::runtime_error::RuntimeError;
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
            return Err(RuntimeError::CannotRedefine(name, name_wrapped_node.loc));
        }

        let value_wrapped_node = eval::eval_node(env, args.remove(0))?;

        env.insert(name, value_wrapped_node);
        Ok(WrappedNode::new(Node::Number(0), name_wrapped_node.loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedNode(
            "symbol".to_string(),
            name_wrapped_node.node,
            name_wrapped_node.loc,
        ))
    }
}

pub fn eval_special_update(
    env: &mut Env,
    mut args: Vec<WrappedNode>,
) -> Result<WrappedNode, RuntimeError> {
    let name_wrapped_node = args.remove(0);
    let node = name_wrapped_node.node;
    let loc = name_wrapped_node.loc;

    if let Node::Symbol(name) = node {
        if !env.exists(&name) {
            return Err(RuntimeError::CannotUpdateUndefinedName(name, loc));
        }
        let value = args.remove(0);
        env.insert(name, value);
        Ok(WrappedNode::new(Node::Number(0), loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedNode(
            "symbol".to_string(),
            node,
            loc,
        ))
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
            return Err(RuntimeError::CannotUpdateUndefinedName(name, loc));
        }

        let _index_node = args.remove(0);
        let value_node = eval::eval_node(env, args.remove(0))?;

        if let Some(entry) = env.remove(&name) {
            match entry.node {
                Node::List { mut children } => {
                    //TODO: get num from index_node instead of using zero
                    children[0] = value_node;
                    env.insert(name, WrappedNode::new(Node::List { children }, loc.clone()));
                }
                _ => {
                    return Err(RuntimeError::CannotUpdateElementInNode(entry.node, loc));
                }
            }
        }

        Ok(WrappedNode::new(Node::Number(0), loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedNode(
            "symbol".to_string(),
            node,
            loc,
        ))
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
        _ => Err(RuntimeError::UnexpectedNode(
            "list of parameters".to_string(),
            param_list.node,
            param_list.loc,
        )),
    }
}
