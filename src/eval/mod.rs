mod env;
mod primitives;
mod specials;

use self::env::Env;
use ast::Node;

pub fn create_root_env() -> Env {
    let env = Env::new();
    primitives::init_env_with_primitives(&env);
    env
}

pub fn eval(env: &mut Env, nodes: Vec<Node>) -> Result<Node, String> {
    let mut output_node = Node::Error("NO-INPUT".to_string()); // TODO: should this be nil?

    for node in nodes {
        output_node = eval_node(env, node)?;
    }

    Ok(output_node)
}

fn eval_node(env: &mut Env, node: Node) -> Result<Node, String> {
    match node {
        Node::List(list_node) => eval_list(env, list_node.children),
        Node::Symbol(name) => match env.get(&name) {
            Some(&ref node) => Ok(node.clone()),
            None => Err(format!("Undefined name: {}", name)),
        },
        n @ Node::Number(_) => Ok(n),
        n => Err(format!("Unable to eval node: {}", n.display())),
    }
}

fn eval_list(env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    match children.remove(0) {
        Node::Symbol(ref name) => match name.as_ref() {
            "list" => specials::eval_special_list(env, children),
            "quote" => specials::eval_special_quote(children),
            "def" => specials::eval_special_def(env, children),
            "fn" => specials::eval_special_fn(env, children),
            "update!" => specials::eval_special_update(env, children),
            "update-element!" => specials::eval_special_update_element(env, children),
            _ => Err(format!(
                "Don't know what to do with list starting with: {}",
                name
            )),
        },
        n => {
            let evaluated_head = eval_node(env, n)?;

            match evaluated_head {
                Node::Proc(proc_node) => {
                    let mut body = proc_node.body;
                    Ok(body.remove(0)) // TODO: we currently just return the first item in the body
                }
                _ => Err(format!(
                    "Don't know what to do with list starting with: {}",
                    evaluated_head.display()
                )),
            }
        }
    }
}
