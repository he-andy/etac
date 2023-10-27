use crate::{
    cfg::{SSANode, CFG},
    translate_types::LIRNode,
};
use std::collections::HashMap;

fn replace_copies(node: &mut LIRNode, rep_map: &HashMap<String, String>) {
    match node {
        LIRNode::Move {
            idx: _,
            val: (dest, src),
        } => {
            replace_copies(dest, rep_map);
            replace_copies(src, rep_map);
        }
        LIRNode::Call {
            idx: _,
            val: (_, n, _),
        } => {
            for arg in n {
                replace_copies(arg, rep_map);
            }
        }
        LIRNode::CJump {
            idx: _,
            val: (n, _),
        } => replace_copies(n, rep_map),
        LIRNode::Return { idx: _, val: n } => {
            for ret in n {
                replace_copies(ret, rep_map);
            }
        }
        LIRNode::Temp { idx: i, val: n } => {
            if rep_map.contains_key(n) {
                *node = LIRNode::Temp {
                    idx: *i,
                    val: rep_map[n].clone(),
                };
            }
        }
        LIRNode::Op {
            idx: _,
            val: (_, n1, n2),
        } => {
            replace_copies(n1, rep_map);
            replace_copies(n2, rep_map);
        }
        LIRNode::Mem { idx: _, val: n } => replace_copies(n, rep_map),
        _ => (),
    }
}

pub fn copy_propagation(cfg: &mut CFG<SSANode<Vec<LIRNode>>>) {
    // Store map of representatives for each temp
    let mut rep_map = HashMap::<String, String>::new();

    for node in cfg.graph.node_indices() {
        //let node_weight = cfg.graph.node_weight(node).unwrap();
        for lir_node in cfg.graph.node_weight_mut(node).unwrap().val.iter_mut() {
            match lir_node {
                LIRNode::Move {
                    idx: _,
                    val: (x, y),
                } => match (&**x, &**y) {
                    (LIRNode::Temp { idx: _, val: t }, LIRNode::Temp { idx: _, val: s }) => {
                        if (s.len() >= 3 && &s[0..3] == "_RV")
                            || (t.len() >= 3 && &t[0..3] == "_RV")
                        {
                            continue;
                        }
                        if (s.len() >= 4 && &s[0..4] == "_ARG")
                            || (t.len() >= 4 && &t[0..4] == "_ARG")
                        {
                            continue;
                        }
                        let rep = if rep_map.contains_key(s) {
                            rep_map[s].clone()
                        } else {
                            s.clone()
                        };
                        rep_map.insert(t.clone(), rep);
                    }
                    _ => replace_copies(lir_node, &rep_map),
                },
                _ => replace_copies(lir_node, &rep_map),
            }
        }
    }
}
