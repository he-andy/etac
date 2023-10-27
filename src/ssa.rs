use crate::cfg::CFGNode;
use crate::cfg::SSANode;
use crate::cfg::CFG;
use crate::next_int;
use crate::translate_types::LIRNode;
use petgraph::algo;
use petgraph::algo::dominators::Dominators;
use petgraph::{graph::NodeIndex, Direction};
use std::collections::{HashMap, HashSet};
use std::vec;

/// Build dominance frontier
fn compute_dominance_frontier<T: Clone + std::fmt::Debug + CFGNode>(
    dominators: &Dominators<NodeIndex>,
    cfg: &CFG<T>,
) -> Vec<HashSet<NodeIndex>> {
    //computes all nodes dominated by node
    fn dom_by(
        node: NodeIndex,
        dominators: &Dominators<NodeIndex>,
        memo: &mut Vec<Option<HashSet<NodeIndex>>>,
    ) {
        if memo[node.index()].is_some() {
            return;
        }

        let mut dom = HashSet::from([node]);
        for idom in dominators.immediately_dominated_by(node) {
            if idom.index() == node.index() {
                continue;
            }
            dom_by(idom, dominators, memo);
            dom.extend(memo[idom.index()].as_ref().unwrap());
        }
        memo[node.index()] = Some(dom);
    }

    //helper function to compute the dominance frontier of each node
    fn dom_frontier_recur<T: Clone + std::fmt::Debug + CFGNode>(
        node: NodeIndex,
        dominators: &Dominators<NodeIndex>,
        dom_by: &Vec<HashSet<NodeIndex>>,
        cfg: &CFG<T>,
        memo: &mut Vec<Option<HashSet<NodeIndex>>>,
    ) {
        if memo[node.index()].is_some() {
            return;
        }

        let mut DF: HashSet<NodeIndex> = cfg
            .graph
            .neighbors_directed(node, Direction::Outgoing)
            .collect::<HashSet<_>>();

        for idom in dominators.immediately_dominated_by(node) {
            if idom.index() == node.index() {
                continue;
            }
            dom_frontier_recur(idom, dominators, dom_by, cfg, memo);
            DF.extend(memo[idom.index()].as_ref().unwrap());
        }

        memo[node.index()] = Some(
            DF.difference(&dom_by[node.index()])
                .map(|x| *x)
                .collect::<HashSet<_>>(),
        );
    }

    let mut memo = vec![None; cfg.graph.node_count()];
    dom_by(cfg.start(), dominators, &mut memo);
    let dom_by = memo.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut memo = vec![None; cfg.graph.node_count()];
    dom_frontier_recur(cfg.start(), dominators, &dom_by, cfg, &mut memo);
    memo.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>()
}

impl CFG<SSANode<Vec<LIRNode>>> {
    pub fn convert_to_normal(&mut self) {
        //let mut to_remove = HashSet::new();
        for node in self.graph.node_indices() {
            let node_weight = self.graph.node_weight_mut(node).unwrap();
            let mut new_nodes = HashMap::<NodeIndex, Vec<LIRNode>>::new();
            let node_weight = self.graph.node_weight(node).unwrap();
            for (defname, defnum) in &node_weight.phi {
                for in_node in self.graph.neighbors_directed(node, Direction::Incoming) {
                    let last_def = self
                        .graph
                        .node_weight(in_node)
                        .unwrap()
                        .last_def
                        .get(defname);
                    match last_def {
                        None => {
                            let new_def = LIRNode::Move {
                                idx: 0,
                                val: (
                                    LIRNode::Temp {
                                        idx: 0,
                                        val: format!("_ssagen_{}_{}", defname, defnum),
                                    }
                                    .into(),
                                    LIRNode::Const { idx: 0, val: 74 }.into(),
                                ),
                            };
                            new_nodes
                                .entry(in_node)
                                .and_modify(|f| f.push(new_def.clone()))
                                .or_insert(vec![new_def]);
                        }
                        Some(n) => {
                            let new_def = LIRNode::Move {
                                idx: 0,
                                val: (
                                    LIRNode::Temp {
                                        idx: 0,
                                        val: format!("_ssagen_{}_{}", defname, defnum),
                                    }
                                    .into(),
                                    LIRNode::Temp {
                                        idx: 0,
                                        val: format!("_ssagen_{}_{}", defname, n),
                                    }
                                    .into(),
                                ),
                            };
                            new_nodes
                                .entry(in_node)
                                .and_modify(|f| f.push(new_def.clone()))
                                .or_insert(vec![new_def]);
                        }
                    }
                }
            }

            for (k, mut v) in new_nodes {
                if !v.is_empty() {
                    let prev_node = &mut self.graph.node_weight_mut(k).unwrap().val;
                    match prev_node.last() {
                        Some(LIRNode::Jump { .. }) | Some(LIRNode::CJump { .. }) => {
                            let jump = prev_node.pop().unwrap();
                            prev_node.extend(v);
                            prev_node.push(jump);
                        }
                        Some(_) => {
                            prev_node.extend(v);
                        }
                        None => {}
                    }

                    // let weight = *self
                    //     .graph
                    //     .edge_weight(self.graph.find_edge(k, node).unwrap())
                    //     .unwrap();

                    // let jump_to = &mut self.graph.node_weight_mut(node).unwrap().val;
                    // match jump_to.is_label() {
                    //     Some(l) => {
                    //         let label = format!("{}_{}", l, next_int());
                    //         v.insert(
                    //             0,
                    //             LIRNode::Label {
                    //                 idx: 0,
                    //                 val: label.clone(),
                    //             }
                    //             .into(),
                    //         );
                    //         v.push(LIRNode::Jump { idx: 0, val: l });
                    //         let jump_from = self
                    //             .graph
                    //             .node_weight_mut(k)
                    //             .unwrap()
                    //             .val
                    //             .last_mut()
                    //             .unwrap();
                    //         match jump_from {
                    //             LIRNode::Jump { idx, .. } => {
                    //                 *jump_from = LIRNode::Jump {
                    //                     idx: *idx,
                    //                     val: label.clone(),
                    //                 };
                    //             }
                    //             LIRNode::CJump {
                    //                 idx,
                    //                 val: (cond, _),
                    //             } => {
                    //                 //if the edge weight is true, then relabel the jump
                    //                 if weight {
                    //                     *jump_from = LIRNode::CJump {
                    //                         idx: *idx,
                    //                         val: (cond.clone(), label.clone()),
                    //                     };
                    //                 }
                    //             }
                    //             _ => (),
                    //         }
                    //     }
                    //     None => (),
                    // }

                    // let new_node = self.graph.add_node(SSANode {
                    //     val: v,
                    //     last_def: HashMap::new(),
                    //     phi: HashMap::new(),
                    // });

                    // self.graph.add_edge(k, new_node, weight);
                    // self.graph.add_edge(new_node, node, false);
                    // to_remove.insert(self.graph.find_edge(k, node).unwrap());
                }
            }
        }
        //self.graph.retain_edges(|_, x| !to_remove.contains(&x));
    }

    pub fn convert_to_ssa(cfg: &CFG<Vec<LIRNode>>) -> Self {
        let mut ssa_cfg = cfg.wrap_SSA();
        ssa_cfg.delete_unreachable();

        let dominators = algo::dominators::simple_fast(&ssa_cfg.graph, ssa_cfg.start());

        let dominator_frontier: Vec<HashSet<NodeIndex>> =
            compute_dominance_frontier(&dominators, &ssa_cfg);

        //use dominance frontier to place phi nodes
        let defsites = &mut ssa_cfg.defs;
        let vars = defsites.keys().cloned().collect::<Vec<_>>();
        for var in vars {
            let mut worklist = vec![];
            for node in defsites[&var].iter() {
                worklist.push(*node);
            }
            while let Some(node) = worklist.pop() {
                for y in &dominator_frontier[node.index()] {
                    ssa_cfg
                        .graph
                        .node_weight_mut(*y)
                        .unwrap()
                        .phi
                        .insert(var.to_string(), 0);
                    // .entry(var.to_string())
                    // .and_modify(|x| *x += 1)
                    // .or_insert(1);
                    if !defsites[&var].contains(y) {
                        defsites.get_mut(&var).unwrap().insert(*y);
                        worklist.push(*y);
                    }
                }
            }
        }
        //insert phi nodes
        // for node in ssa_cfg.graph.node_weights_mut() {
        //     node.phi.retain(|_, v| *v > 1);
        // }
        let mut stack = ssa_cfg
            .defs
            .keys()
            .map(|x| (x.to_string(), vec![]))
            .collect::<HashMap<_, _>>();

        ssa_cfg.rename_ssa(ssa_cfg.start(), &dominators, &mut stack);
        for node in ssa_cfg.graph.node_weights_mut() {
            for instr in node.val.iter_mut() {
                instr.rename_phi_temp();
            }
        }

        for node in ssa_cfg.graph.node_indices() {
            let mut walker = ssa_cfg
                .graph
                .neighbors_directed(node, Direction::Incoming)
                .detach();
            let phis = ssa_cfg.graph.node_weight(node).unwrap().phi.keys().clone();
            let mut phi_uses = HashMap::new();
            for phi in phis {
                while let Some(pred) = walker.next_node(&ssa_cfg.graph) {
                    if let Some(last_def) =
                        ssa_cfg.graph.node_weight(pred).unwrap().last_def.get(phi)
                    {
                        phi_uses
                            .entry(phi.clone())
                            .or_insert(vec![])
                            .push(*last_def);
                    }
                }
            }

            ssa_cfg.graph.node_weight_mut(node).unwrap().phi_uses = phi_uses;
        }

        ssa_cfg
    }

    fn rename_ssa(
        &mut self,
        block: NodeIndex,
        dominators: &Dominators<NodeIndex>,
        stack: &mut HashMap<String, Vec<usize>>,
    ) {
        let mut update_counts = HashMap::<String, usize>::new();

        let updates = self.graph.node_weight_mut(block).unwrap().update_phi();
        for (var, idx) in updates {
            stack.get_mut(&var).unwrap().push(idx);
            update_counts
                .entry(var)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        for instr in self.graph.node_weight_mut(block).unwrap().val.iter_mut() {
            instr.rename_use(&stack);
            let update = instr.rename_def();
            if update.is_some() {
                let (var, idx) = update.unwrap();
                stack.get_mut(&var).unwrap().push(idx);
                update_counts
                    .entry(var)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }

        for (var, s) in stack.iter() {
            match s.last() {
                Some(n) => {
                    self.graph
                        .node_weight_mut(block)
                        .unwrap()
                        .last_def
                        .insert(var.clone(), *n);
                }
                None => (),
            }
        }

        for idom in dominators.immediately_dominated_by(block) {
            if idom.index() == block.index() {
                continue;
            }
            self.rename_ssa(idom, dominators, stack);
        }

        //backtrack changes to stack
        for (var, count) in update_counts {
            let var_stack = stack.get_mut(&var).unwrap();
            for _ in 0..count {
                var_stack.pop();
            }
        }
    }
}

impl LIRNode {
    fn rename_use(&mut self, stack: &HashMap<String, Vec<usize>>) {
        match self {
            LIRNode::Move {
                val: (dest, src), ..
            } => match &**dest {
                LIRNode::Temp { .. } | LIRNode::Name { .. } => src.rename_use(stack),
                _ => {
                    dest.rename_use(stack);
                    src.rename_use(stack);
                }
            },
            LIRNode::Call {
                val: (_, args, _), ..
            } => {
                for arg in args {
                    arg.rename_use(stack);
                }
            }
            LIRNode::Jump { .. } => (),
            LIRNode::CJump { val: (cond, _), .. } => cond.rename_use(stack),
            LIRNode::Return { val, .. } => {
                for node in val.iter_mut() {
                    node.rename_use(stack);
                }
            }
            LIRNode::Label { .. } => (),
            LIRNode::Const { .. } => (),
            LIRNode::Temp { idx, val } => {
                if let Some(_) = crate::tiles::is_return(&val) {
                    return ();
                }
                if val.starts_with("_ARG") {
                    return ();
                }
                match stack.get(val) {
                    Some(v) => {
                        *self = LIRNode::SSA_Temp {
                            idx: *idx,
                            val: (val.clone(), v.last().unwrap().clone()),
                        }
                    }
                    None => (),
                }
            }
            LIRNode::SSA_Temp { .. } => (),
            LIRNode::Op { val: (_, l, r), .. } => {
                l.rename_use(stack);
                r.rename_use(stack);
            }
            LIRNode::Mem { val, .. } => val.rename_use(stack),
            LIRNode::Name { idx, val } => match stack.get(val) {
                Some(v) => {
                    *self = LIRNode::SSA_Temp {
                        idx: *idx,
                        val: (val.clone(), v.last().unwrap().clone()),
                    }
                }
                None => (),
            },
        }
    }

    fn rename_def(&mut self) -> Option<(String, usize)> {
        match self {
            LIRNode::Move {
                val: (dest, ..), ..
            } => match **dest {
                LIRNode::Temp { .. } | LIRNode::Name { .. } => dest.rename_def(),
                _ => None,
            },
            LIRNode::Name { val, .. } => {
                let val = val.clone();
                let next = next_int();
                *self = LIRNode::SSA_Temp {
                    idx: 0,
                    val: (val.clone(), next),
                };
                Some((val, next))
            }
            LIRNode::Temp { idx, val } => {
                if let Some(_) = crate::tiles::is_return(&val) {
                    return None;
                }
                let val = val.clone();
                let next = next_int();
                *self = LIRNode::SSA_Temp {
                    idx: *idx,
                    val: (val.clone(), next),
                };
                Some((val, next))
            }
            _ => None,
        }
    }

    fn rename_phi_temp(&mut self) {
        match self {
            LIRNode::Move { val: (l, r), .. } => {
                l.rename_phi_temp();
                r.rename_phi_temp();
            }
            LIRNode::Call {
                val: (_, args, _), ..
            } => {
                for arg in args {
                    arg.rename_phi_temp();
                }
            }
            LIRNode::Jump { val, .. } => (),
            LIRNode::CJump { val: (cond, _), .. } => cond.rename_phi_temp(),
            LIRNode::Return { val: returns, .. } => {
                for ret in returns {
                    ret.rename_phi_temp();
                }
            }
            LIRNode::Label { .. } => (),
            LIRNode::Const { .. } => (),
            LIRNode::Temp { .. } => (),
            LIRNode::SSA_Temp { idx, val: (def, n) } => {
                *self = LIRNode::Temp {
                    idx: *idx,
                    val: format!("_ssagen_{}_{}", def, n),
                }
            }
            LIRNode::Op { val: (_, l, r), .. } => {
                l.rename_phi_temp();
                r.rename_phi_temp();
            }
            LIRNode::Mem { idx, val } => val.rename_phi_temp(),
            LIRNode::Name { idx, val } => (),
        }
    }
}

fn traverse_and_renumber(node: &mut LIRNode, n: &mut usize) {
    *n += 1;
    match node {
        LIRNode::Move { idx, val: (l, r) } => {
            *idx = *n;
            traverse_and_renumber(l, n);
            traverse_and_renumber(r, n);
        }
        LIRNode::Call {
            idx,
            val: (_, val, _),
        } => {
            *idx = *n;
            for node in val.iter_mut() {
                traverse_and_renumber(node, n);
            }
        }
        LIRNode::Jump { idx, .. } => {
            *idx = *n;
        }
        LIRNode::CJump {
            idx,
            val: (cond, _),
        } => {
            *idx = *n;
            traverse_and_renumber(cond, n);
        }
        LIRNode::Return { idx, val } => {
            *idx = *n;
            for node in val.iter_mut() {
                traverse_and_renumber(node, n);
            }
        }
        LIRNode::Label { idx, .. } => {
            *idx = *n;
        }

        LIRNode::Const { idx, .. } => {
            *idx = *n;
        }
        LIRNode::Temp { idx, .. } => {
            *idx = *n;
        }
        LIRNode::SSA_Temp { idx, .. } => {
            *idx = *n;
        }
        LIRNode::Op {
            idx,
            val: (_, l, r),
        } => {
            *idx = *n;
            traverse_and_renumber(l, n);
            traverse_and_renumber(r, n);
        }
        LIRNode::Mem { idx, val } => {
            *idx = *n;
            traverse_and_renumber(val, n);
        }
        LIRNode::Name { idx, .. } => {
            *idx = *n;
        }
    }
}

pub fn renumber_lir(code: &mut Vec<LIRNode>) -> usize {
    let mut n = 1;
    for node in code.iter_mut() {
        traverse_and_renumber(node, &mut n);
    }
    n + 1
}

pub fn renumber_cfg(cfg: &mut CFG<SSANode<Vec<LIRNode>>>) {
    let mut n = 1;
    for node in cfg.graph.node_weights_mut() {
        for node in node.val.iter_mut() {
            traverse_and_renumber(node, &mut n);
        }
    }
}
