use crate::{
    cfg::{CFGNode, Dir, SSANode, Var, CFG},
    translate_types::LIRNode,
};
use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn union<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.union(b).cloned().collect()
}

fn intersection<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.intersection(b).cloned().collect()
}

fn live_var_transfer<T: CFGNode + Clone + Debug>(
    l: &HashSet<Var>,
    n: &NodeIndex,
    cfg: &CFG<T>,
) -> HashSet<Var> {
    let n = cfg.graph.node_weight(*n).unwrap();
    n.uses().union(&(l - &n.defs())).cloned().collect()
}

pub fn live_variable_analysis<T: CFGNode + Clone + std::fmt::Debug>(
    cfg: &CFG<T>,
) -> Vec<HashSet<Var>> {
    cfg.work_list(
        union,
        live_var_transfer,
        HashSet::<Var>::new(),
        HashSet::<Var>::new(),
        Dir::Backward,
        true,
    )
}

fn reaching_definitions_transfer(
    l: &HashSet<NodeIndex>,
    n: &NodeIndex,
    cfg: &CFG<LIRNode>,
) -> HashSet<NodeIndex> {
    let node_weight = cfg.graph.node_weight(*n).unwrap();
    match node_weight {
        LIRNode::Move {
            idx: _,
            val: (x, _),
        } => match &**x {
            LIRNode::Temp { idx: _, val: t } => {
                let gen = HashSet::from([n.clone()]);
                let kill = cfg.def(&Var::Name(t.clone()));
                gen.union(&(l - &kill)).cloned().collect()
            }
            _ => HashSet::new(),
        },
        _ => HashSet::new(),
    }
}

pub fn reaching_definitions_analysis(stmts: Vec<LIRNode>) -> Vec<HashSet<NodeIndex>> {
    let cfg = CFG::new(&stmts);
    // cfg.debug_cfg();
    cfg.work_list(
        union,
        reaching_definitions_transfer,
        HashSet::new(),
        HashSet::new(),
        Dir::Forward,
        false,
    )
}

fn available_copies_transfer(
    l: &HashSet<(String, String)>,
    n: &NodeIndex,
    cfg: &CFG<LIRNode>,
) -> HashSet<(String, String)> {
    let node_weight = cfg.graph.node_weight(*n).unwrap();
    match node_weight {
        LIRNode::Move {
            idx: _,
            val: (x, y),
        } => match (&**x, &**y) {
            (LIRNode::Temp { idx: _, val: t }, LIRNode::Temp { idx: _, val: s }) => {
                let mut gen = HashSet::new();
                gen.insert((t.clone(), s.clone()));
                let mut kill = HashSet::new();
                for node in cfg.graph.node_indices() {
                    let inner_node_weight = cfg.graph.node_weight(node).unwrap();
                    match inner_node_weight {
                        LIRNode::Move {
                            idx: _,
                            val: (z, w),
                        } => match (&**z, &**w) {
                            (
                                LIRNode::Temp { idx: _, val: u },
                                LIRNode::Temp { idx: _, val: v },
                            ) => {
                                if u == t {
                                    kill.insert((u.clone(), t.clone()));
                                }
                                if v == t {
                                    kill.insert((v.clone(), t.clone()));
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                gen.union(&(l - &kill)).cloned().collect()
            }
            _ => HashSet::new(),
        },
        _ => HashSet::new(),
    }
}

pub fn available_copies_analysis(
    cfg: &CFG<SSANode<Vec<LIRNode>>>,
) -> Vec<HashSet<(String, String)>> {
    //cfg.debug_cfg();
    let mut top = HashSet::new();

    for var1 in cfg.defs.iter() {
        for var2 in cfg.defs.iter() {
            if var1.0 != var2.0 {
                if let Var::Name(var) = var1.0 {
                    if let Var::Name(other_var) = var2.0 {
                        top.insert((var.clone(), other_var.clone()));
                    }
                }
            }
        }
    }
    return vec![];
    // cfg.work_list(
    //     intersection,
    //     available_copies_transfer,
    //     top,
    //     HashSet::new(),
    //     Dir::Forward,
    //     true,
    // )
}
