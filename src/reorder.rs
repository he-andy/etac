use crate::ir_types::{LIRExpr, LIRStmt};
use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;

type Bblock = (String, Range<usize>);

pub fn reorder(s: Vec<LIRStmt>) -> Vec<LIRStmt> {
    let (s, cfg) = make_cfg(s);
    //trace finding
    let mut marked = HashSet::new();
    let mut counts = HashMap::new();
    let mut traces = vec![];
    loop {
        if let Some(source) = heuristic(cfg.node_indices(), &marked, &counts) {
            let mut memo = HashMap::new();
            find_maximal_trace(&mut marked, &mut memo, source, &cfg);
            let trace = reconstruct_trace(&memo, source);
            update_counts(&trace, &memo, &mut counts);
            for block in trace.iter() {
                marked.insert(*block);
            }
            traces.push(trace);
        } else {
            break;
        }
    }
    //traces.sort_by_key(|x| x.first().unwrap().index());
    reorder_traces(&mut traces);
    let reordered = fix_jumps(&traces, &cfg, s);
    remove_extra_labels(reordered)
}

/// generates a cfg from a vector of statements by parsing into basic blocks
#[inline]
fn make_cfg(s: Vec<LIRStmt>) -> (Vec<LIRStmt>, Graph<Bblock, bool>) {
    let mut cfg = Graph::<Bblock, bool>::new();
    let mut basic_blocks = vec![];
    let mut i = 0;
    for (j, stmt) in s.iter().enumerate() {
        match stmt {
            LIRStmt::Label(_) => {
                if j > i {
                    let label = if let LIRStmt::Label(l) = &s[i] {
                        l.clone()
                    } else {
                        format!("__{i}")
                    };
                    basic_blocks.push((label, i..j));
                    i = j;
                }
            }
            LIRStmt::CJump(_, _, _) | LIRStmt::Jump(_) | LIRStmt::Return(_) => {
                let label = if let LIRStmt::Label(l) = &s[i] {
                    l.clone()
                } else {
                    format!("__{i}")
                };
                basic_blocks.push((label, i..(j + 1)));
                i = j + 1;
            }
            _ => (),
        }
    }

    if i < s.len() {
        let label = if let LIRStmt::Label(l) = &s[i] {
            l.clone()
        } else {
            format!("__{i}")
        };
        basic_blocks.push((label.clone(), i..s.len()));
    }

    let mut node_idx_lookup = HashMap::new();

    let max_node = basic_blocks.len() - 1;
    //add the nodes into the graph
    for (label, block) in basic_blocks {
        node_idx_lookup.insert(label.clone(), cfg.add_node((label, block.clone())));
    }

    for node in cfg.node_indices() {
        let weight = cfg.node_weight(node).unwrap();
        match &s[weight.1.clone().last().unwrap()] {
            LIRStmt::Jump(j) => match j {
                LIRExpr::Name(s) => {
                    let n_node = node_idx_lookup.get(s).unwrap();
                    cfg.add_edge(node, *n_node, true);
                }
                _ => unreachable!("nonono this can't be happening"),
            },
            LIRStmt::CJump(_, t, f) => {
                if let Some(f_node) = node_idx_lookup.get(f) {
                    cfg.add_edge(node, *f_node, false);
                }
                let t_node = node_idx_lookup.get(t).unwrap();
                cfg.add_edge(node, *t_node, true);
            }
            LIRStmt::Return(_) => (),
            _ => {
                if node.index() != max_node {
                    cfg.add_edge(node, (node.index() as u32 + 1).into(), true);
                }
            }
        }
    }
    (s, cfg)
}

/// returns the minimum predecessor count (unmarked) node if one exists, None otherwise
#[inline]
fn heuristic(
    nodes: petgraph::graph::NodeIndices,
    marked: &HashSet<NodeIndex>,
    counts: &HashMap<NodeIndex, usize>,
) -> Option<NodeIndex> {
    if nodes.len() == marked.len() {
        None
    } else {
        nodes.min_by_key(|x| {
            if marked.contains(x) {
                (std::usize::MAX, 0)
            } else {
                match counts.get(x) {
                    None => (0, x.index()),
                    Some(v) => (*v, x.index()),
                }
            }
        })
    }
}

/// returns the length of the maximal trace through unmarked nodes starting from node [source]
fn find_maximal_trace(
    marked: &mut HashSet<NodeIndex>,
    memo: &mut HashMap<NodeIndex, (i32, Option<NodeIndex>)>,
    source: NodeIndex,
    cfg: &Graph<Bblock, bool>,
) -> i32 {
    marked.insert(source);
    let entry = match cfg
        .neighbors(source)
        .into_iter()
        .map(|x| {
            // println!(
            //     "{} -> {}",
            //     source.index(),
            //     cfg.edge_weight(cfg.find_edge(source, x).unwrap()).unwrap()
            // );
            if !marked.contains(&x) {
                match memo.get(&x) {
                    Some((trace_len, _)) => (trace_len.clone(), x),
                    None => (find_maximal_trace(marked, memo, x, cfg), x),
                }
            } else {
                (-1, x)
            }
        })
        .filter(|(v, _)| v >= &0)
        .max()
    {
        Some((trace_len, next)) => (trace_len + 1, Some(next)),
        None => (1, None),
    };
    marked.remove(&source);
    memo.insert(source, entry);
    return entry.0;
}

#[inline]
fn reconstruct_trace(
    memo: &HashMap<NodeIndex, (i32, Option<NodeIndex>)>,
    source: NodeIndex,
) -> Vec<NodeIndex> {
    let mut trace = vec![source];
    let mut next = memo.get(&source).unwrap().1;
    while next != None {
        trace.push(next.unwrap());
        next = memo.get(&next.unwrap()).unwrap().1;
    }
    trace
}

#[inline]
fn update_counts(
    trace: &Vec<NodeIndex>,
    memo: &HashMap<NodeIndex, (i32, Option<NodeIndex>)>,
    counts: &mut HashMap<NodeIndex, usize>,
) {
    let l = trace.len();
    for n in memo.keys() {
        match counts.get(n) {
            Some(v) => counts.insert(*n, v + l),
            None => counts.insert(*n, l),
        };
    }
}

/// given a vector of traces, fix jumps between them and return as a vector of statements
#[inline]
fn fix_jumps(
    traces: &Vec<Vec<NodeIndex>>,
    cfg: &Graph<Bblock, bool>,
    stmts: Vec<LIRStmt>,
) -> Vec<LIRStmt> {
    //iterates over windows of size 2 (equivalent to iter.windows(2) but returns tuples instead)
    let mut x = traces
        .iter()
        .zip(traces.iter().skip(1))
        .map(|(curr, next)| {
            let mut b = fix_trace_jumps(curr, &cfg, &stmts);
            //if curr should not fall through to the next trace, add jump to correct block
            let neighbors = cfg
                .neighbors(*curr.last().unwrap())
                .collect::<Vec<NodeIndex>>();
            if neighbors.len() == 1 {
                let first = *neighbors.first().unwrap();
                if first != *next.first().unwrap() {
                    b.push(LIRStmt::Jump(LIRExpr::Name(
                        cfg.node_weight(first).unwrap().0.clone(),
                    )));
                }
            }
            b
        })
        .chain(std::iter::once(fix_trace_jumps(
            traces.last().unwrap(),
            cfg,
            &stmts,
        )))
        .flatten()
        .collect::<Vec<LIRStmt>>();
    //fix jump for the last case
    let neighbors = cfg
        .neighbors(*traces.last().unwrap().last().unwrap())
        .collect::<Vec<NodeIndex>>();
    if neighbors.len() == 1 {
        let next = &cfg.node_weight(*neighbors.first().unwrap()).unwrap().0;
        x.push(LIRStmt::Jump(LIRExpr::Name(next.clone())))
    }
    x
}

/// removes extra jumps and reorder CJump within a trace
#[inline]
fn fix_trace_jumps(
    trace: &Vec<NodeIndex>,
    cfg: &Graph<Bblock, bool>,
    stmts: &Vec<LIRStmt>,
) -> Vec<LIRStmt> {
    trace
        .iter()
        .zip(trace.iter().skip(1))
        .map(|(first, second)| {
            let mut b = stmts[cfg.node_weight(*first).unwrap().1.clone()].to_vec();
            match b.last().unwrap().clone() {
                LIRStmt::Jump(_) => {
                    b.pop();
                }
                LIRStmt::CJump(e, t, f) => {
                    b.pop();
                    let edge_weight = cfg
                        .edge_weight(cfg.find_edge(*first, *second).unwrap())
                        .unwrap();
                    // if the next block is the true condition, invert to fall through
                    if *edge_weight {
                        let inverted = LIRExpr::Op(
                            crate::ir_types::Op::Xor,
                            Box::new(LIRExpr::Const(1)),
                            Box::new(e.clone()),
                        );
                        b.push(LIRStmt::CJump2(inverted, f.clone()))
                    } else {
                        b.push(LIRStmt::CJump2(e.clone(), t.clone()))
                    };
                }
                _ => (),
            };
            b
        })
        .chain(std::iter::once({
            let mut b = stmts[cfg.node_weight(*trace.last().unwrap()).unwrap().1.clone()].to_vec();
            //remove ending jump
            if let LIRStmt::Jump(_) = b.last().unwrap() {
                b.pop();
            }
            b
        }))
        .flatten()
        .collect()
}

/// remove all LIRStmt::Label that are unused
#[inline]
fn remove_extra_labels(stmts: Vec<LIRStmt>) -> Vec<LIRStmt> {
    let used_labels = stmts
        .iter()
        .map(|x| match x {
            LIRStmt::CJump2(_, t) => vec![t],
            LIRStmt::Jump(t) => match t {
                LIRExpr::Name(s) => vec![s],
                _ => unreachable!("nonono"),
            },
            _ => vec![],
        })
        .flatten()
        .cloned()
        .collect::<HashSet<_>>();

    stmts
        .into_iter()
        .filter(|x| match x {
            LIRStmt::Label(s) => used_labels.contains(s),
            _ => true,
        })
        .collect()
}

/// reorder a nonempty vector of traces to minimize extra jumps
#[inline]
fn reorder_traces(trace_list: &mut Vec<Vec<NodeIndex>>) {
    let start = (0..trace_list.len())
        .min_by_key(|x| trace_list[*x].first().unwrap().index())
        .unwrap();
    trace_list.swap(0, start);
    for i in 1..trace_list.len() {
        let last = trace_list[i - 1].last().unwrap().index();
        let min = (i..trace_list.len()).min_by_key(|x| {
            let first = trace_list[*x].first().unwrap().index();
            if first < last {
                std::usize::MAX
            } else {
                first - last
            }
        });
        match min {
            Some(v) => trace_list.swap(i, v),
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ir_types::{LIRDest, LIRExpr, LIRStmt},
        reorder::reorder,
    };

    #[test]
    fn test_cjump_relabel() {
        let input = vec![
            LIRStmt::Label("l0".into()),
            LIRStmt::CJump(LIRExpr::Const(1), "l2".into(), "l3".into()),
            LIRStmt::Label("l1".into()),
            LIRStmt::Move(LIRDest::Temp("x".into()), LIRExpr::Name("y".into())),
            LIRStmt::Label("l2".into()),
            LIRStmt::Move(LIRDest::Temp("x".into()), LIRExpr::Name("z".into())),
            LIRStmt::Jump(LIRExpr::Name("l1".into())),
            LIRStmt::Label("l3".into()),
            LIRStmt::Move(LIRDest::Temp("x".into()), LIRExpr::Name("a".into())),
            LIRStmt::Return(vec![]),
        ];
        let expected = "[CJump2(Op(Xor, Const(1), Const(1)), \"l3\"), Label(\"l2\"), Move(Temp(\"x\"), Name(\"z\")), Move(Temp(\"x\"), Name(\"y\")), Jump(Name(\"l2\")), Label(\"l3\"), Move(Temp(\"x\"), Name(\"a\")), Return([])]";
        assert_eq!(format!("{:?}", reorder(input)), expected);
    }
    #[test]
    fn test_elim_extra_jumps() {
        let input = vec![
            LIRStmt::Move(LIRDest::Temp("x".into()), LIRExpr::Name("y".into())),
            LIRStmt::Jump(LIRExpr::Name("l1".into())),
            LIRStmt::Label("l2".into()),
            LIRStmt::Move(LIRDest::Temp("y".into()), LIRExpr::Name("y".into())),
            LIRStmt::Jump(LIRExpr::Name("end".into())),
            LIRStmt::Label("l1".into()),
            LIRStmt::Move(LIRDest::Temp("z".into()), LIRExpr::Name("y".into())),
            LIRStmt::Jump(LIRExpr::Name("l2".into())),
            LIRStmt::Label("end".into()),
        ];
        let expected = "[Move(Temp(\"x\"), Name(\"y\")), Move(Temp(\"z\"), Name(\"y\")), Move(Temp(\"y\"), Name(\"y\"))]";
        assert_eq!(format!("{:?}", reorder(input)), expected);
    }
}

/// creates a debuggable cfg
fn debug_cfg(cfg: &Graph<Bblock, bool>, s: &Vec<LIRStmt>) {
    let mut debug_cfg = Graph::<Vec<LIRStmt>, bool>::new();
    for node in cfg.node_weights() {
        debug_cfg.add_node(s[node.1.clone()].to_vec());
    }
    for edge in cfg.edge_indices() {
        let (a, b) = cfg.edge_endpoints(edge).unwrap();
        let weight = cfg.edge_weight(edge).unwrap();
        debug_cfg.add_edge(a, b, *weight);
    }

    println!("{:?}", Dot::new(&debug_cfg))
}
