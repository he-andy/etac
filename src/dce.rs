// //implement dead code removal on the LIR level

// use crate::{
//     cfg::{CFGNode, SSANode, CFG},
//     translate_types::LIRNode,
// };
// use petgraph::graph::NodeIndex;
// use std::collections::{HashMap, HashSet, VecDeque};

// pub fn eliminate_dead_code(cfg: &mut CFG<SSANode<Vec<LIRNode>>>) {
//     //first, find all the definitions
//     let mut defsite = HashSet::new();

//     //denotes a where a variable is used
//     let mut uses = HashMap::new();
//     for node in cfg.graph.node_weights() {
//         for stmt in node.val.iter() {
//             println!("{:?}", stmt);
//             for stmt_uses in stmt.uses() {
//                 uses.entry(stmt_uses.to_string())
//                     .or_insert(HashSet::new())
//                     .insert(stmt.get_idx());
//             }

//             //deal with phi uses
//             for (key, phi_uses) in node.phi_uses.iter() {
//                 for phi_use in phi_uses {
//                     let varname = format!("_ssagen_{}_{}", key, phi_use);
//                     uses.entry(varname.clone())
//                         .or_insert(HashSet::new())
//                         .insert(0);
//                 }
//             }
//         }
//     }

//     let mut worklist = defsite
//         .keys()
//         .map(|x| x.to_string())
//         .collect::<VecDeque<_>>();

//     let mut dead = HashSet::new();
//     while let Some(def) = worklist.pop_front() {
//         let uses_of_def = uses.get(&def);
//         if uses_of_def.is_none() || uses_of_def.unwrap().is_empty() {
//             if let Some(site) = defsite.get(&def) {
//                 dead.insert(site);
//                 println!("dead: {}", def);
//                 println!("site: {}", site);

//                 if let Some(used_to_def) = usesat.get(site) {
//                     for utd in used_to_def {
//                         uses.get_mut(utd)
//                             .unwrap()
//                             .remove(defsite.get(&def).unwrap());
//                         //worklist.push_back(utd.to_string());
//                     }
//                 }
//             }
//         }
//     }

//     //remove dead code
//     for node in cfg.graph.node_weights_mut() {
//         node.val = node
//             .val
//             .iter()
//             .filter(|x| !dead.contains(&x.get_idx()))
//             .cloned()
//             .collect();
//     }
// }
//implement dead code removal on the LIR level

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{cfg::CFGNode, translate_types::LIRNode};

pub fn eliminate_dead_code(lirstmts: Vec<LIRNode>) -> Vec<LIRNode> {
    //denotes a where a variable is used
    let mut uses = HashMap::new();
    //denotes the uses at a given point
    let mut usesat = HashMap::new();
    //denotes where a variable is defined
    let mut defsite = HashMap::new();
    for stmt in &lirstmts {
        for stmt_uses in stmt.uses() {
            uses.entry(stmt_uses.to_string())
                .or_insert(HashSet::new())
                .insert(stmt.get_idx());
            usesat
                .entry(stmt.get_idx())
                .or_insert(HashSet::new())
                .insert(stmt_uses.to_string());
        }
        for def in stmt.defs() {
            uses.entry(def.to_string()).or_insert(HashSet::new());
            let a = defsite.insert(def.to_string(), stmt.get_idx());
            //TODO REMOVE LATER
            //assert!(a.is_none());
        }
    }
    let mut worklist = defsite
        .keys()
        .map(|x| x.to_string())
        .collect::<VecDeque<_>>();

    let mut dead = HashSet::new();
    while let Some(def) = worklist.pop_front() {
        let uses_of_def = uses.get(&def);
        if uses_of_def.is_none() || uses_of_def.unwrap().is_empty() {
            if let Some(site) = defsite.get(&def) {
                dead.insert(site);

                if let Some(used_to_def) = usesat.get(site) {
                    for utd in used_to_def {
                        uses.get_mut(utd)
                            .unwrap()
                            .remove(defsite.get(&def).unwrap());
                        worklist.push_back(utd.to_string());
                    }
                }
            }
        }
    }

    lirstmts
        .into_iter()
        .filter(|x| !dead.contains(&x.get_idx()))
        .collect()
}
