use crate::analysis;
use crate::cfg;
use crate::cfg::CFGNode;
use crate::cfg::Var;
use crate::cfg::CFG;
use crate::ir_types::LIRStmt;
use crate::ir_types::{LIRCompUnit, LIRFuncDecl};
use crate::translate_types::LIRNode;
use petgraph::algo::tarjan_scc;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::collections::HashSet;

fn find_loops(cfg: &CFG<Vec<LIRNode>>) -> Vec<Vec<NodeIndex>> {
    let sccs = tarjan_scc(&cfg.graph);
    let mut loops = Vec::new();

    for scc in sccs {
        if scc.len() > 1 {
            // If the SCC has more than one node, consider it as a loop.
            loops.push(scc);
        } else {
            // If the SCC has one node, check if it has a self-loop.
            let node = scc[0];
            if cfg
                .graph
                .edges(node)
                .any(|edge| edge.source() == edge.target())
            {
                loops.push(scc);
            }
        }
    }

    println!("Loops: {:?}", loops);
    loops
}

fn is_invariant(
    cfg: &CFG<Vec<LIRNode>>,
    loop_nodes: &HashSet<NodeIndex>,
    exprs: &Vec<LIRNode>,
) -> bool {
    for expr in exprs {
        match expr {
            LIRNode::Move { val: (_, r), .. } => {
                if !is_invariant(cfg, loop_nodes, &vec![*r.clone()]) {
                    return false;
                }
            }
            LIRNode::Call {
                val: (_, args, _), ..
            } => {
                if !args
                    .iter()
                    .all(|arg| is_invariant(cfg, loop_nodes, &vec![arg.clone()]))
                {
                    return false;
                }
            }
            LIRNode::Op { val: (_, l, r), .. } => {
                if !(is_invariant(cfg, loop_nodes, &vec![*l.clone()])
                    && is_invariant(cfg, loop_nodes, &vec![*r.clone()]))
                {
                    return false;
                }
            }
            LIRNode::Mem { .. } => return false, // Assume memory operations are not invariant
            LIRNode::Temp { val, .. } | LIRNode::SSA_Temp { val: (val, _), .. } => {
                let var = Var::Name(val.clone());
                let defs = cfg.def(&var);
                if !defs.iter().all(|&def| {
                    // If a definition is outside the loop or is invariant, then it's fine
                    !loop_nodes.contains(&def)
                        || is_invariant(cfg, loop_nodes, &cfg.graph.node_weight(def).unwrap())
                }) {
                    return false;
                }
            }
            _ => (),
        }
    }
    true
}

fn find_invariants(cfg: &CFG<Vec<LIRNode>>, loops: &Vec<Vec<NodeIndex>>) -> Vec<Vec<NodeIndex>> {
    loops
        .iter()
        .map(|loop_nodes| {
            let loop_nodes_set: HashSet<_> = loop_nodes.iter().copied().collect();
            loop_nodes
                .iter()
                .filter(|&&node| is_invariant(cfg, &loop_nodes_set, &cfg.graph[node]))
                .copied()
                .collect()
        })
        .collect()
}

pub fn loop_invariant_code_motion(mut cfg: &mut CFG<Vec<LIRNode>>) {
    let loops = find_loops(&cfg);
    let invariants = find_invariants(&cfg, &loops);
    hoist_invariants(&mut cfg, &loops, &invariants);

    println!("Invariants {:?}", invariants);
}

fn hoist_invariants(
    cfg: &mut CFG<Vec<LIRNode>>,
    loops: &Vec<Vec<NodeIndex>>,
    invariants: &Vec<Vec<NodeIndex>>,
) {
    todo!()
}
