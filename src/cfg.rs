use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use petgraph::dot::Config::NodeNoLabel;
use petgraph::dot::{Config, Dot};
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::fmt::Write;

use crate::next_int;
use crate::ssa::renumber_lir;
use crate::translate_types::loc;
use crate::translate_types::LIRNode;
use crate::translate_types::OPCODE;
use crate::translate_types::{ins, Register};

#[derive(Ord, PartialOrd, Debug, Clone, Eq, Hash, PartialEq)]
pub enum Var {
    Name(String),
    Reg(Register),
}

impl Var {
    pub fn is_named_reg(&self) -> bool {
        match self {
            Var::Name(_) => false,
            Var::Reg(reg) => reg.is_named(),
        }
    }
    pub fn unwrap_reg(&self) -> Register {
        match self {
            Var::Name(_) => panic!("unwrap_reg called on Var::Name"),
            Var::Reg(reg) => reg.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Var::Name(name) => name.clone(),
            Var::Reg(reg) => reg.to_string(),
        }
    }
}

impl loc {
    fn to_var(&self) -> HashSet<Var> {
        match self {
            loc::Deref(a, b, _, _) => match (a, b) {
                (None, None) => HashSet::new(),
                (None, Some(a)) | (Some(a), None) => HashSet::from([Var::Reg(a.clone())]),
                (Some(a), Some(b)) => HashSet::from([Var::Reg(a.clone()), Var::Reg(b.clone())]),
            },
            loc::Literal(_) => HashSet::new(),
            loc::Register(a) => HashSet::from([Var::Reg(a.clone())]),
            loc::Label(_) => HashSet::new(),
            loc::Global(a) => HashSet::from([Var::Name(a.clone())]),
            loc::Null => HashSet::new(),
        }
    }
}

pub enum CF {
    CJump(String),
    Jump(String),
    Return,
    Normal,
}

pub trait CFGNode {
    fn uses(&self) -> HashSet<Var>;
    fn defs(&self) -> HashSet<Var>;
    fn is_label(&self) -> Option<String>;
    fn control_flow(&self) -> CF;
}

pub enum Dir {
    Forward,
    Backward,
}

#[derive(Debug, Clone)]
pub struct CFG<T: CFGNode + Clone + std::fmt::Debug> {
    pub graph: Graph<T, bool>,
    pub defs: HashMap<Var, HashSet<NodeIndex>>,
}

//TODO: reverse postorder shit?
impl<T: CFGNode + Clone + std::fmt::Debug> CFG<T> {
    pub fn new(stmts: &Vec<T>) -> Self {
        let mut graph = Graph::<T, bool>::new();

        //adds nodes to graph and builds a map of all labels
        let label_map = stmts
            .iter()
            .map(|x| {
                let idx = graph.add_node(x.clone());
                (x.is_label(), idx)
            })
            .filter_map(|x| match x.0 {
                Some(label) => Some((label, x.1)),
                None => None,
            })
            .collect::<HashMap<String, NodeIndex>>();

        let mut defs = HashMap::<Var, HashSet<NodeIndex>>::new();
        //Build edges + def table
        for node in graph.node_indices() {
            //build defs table
            for var in graph.node_weight(node).unwrap().defs() {
                match defs.get_mut(&var) {
                    Some(set) => {
                        set.insert(node.clone());
                    }
                    None => {
                        defs.insert(var, HashSet::from([node]));
                    }
                };
            }

            match graph.node_weight(node).unwrap().control_flow() {
                CF::Jump(label) => {
                    let _ = graph.add_edge(node, *label_map.get(&label).unwrap(), false);
                }

                CF::CJump(label) => {
                    if let Some(label) = label_map.get(&label) {
                        graph.add_edge(node, *label, true);
                    }
                    let next = node.index() + 1;
                    if next < stmts.len() {
                        graph.add_edge(node, (next as u32).into(), false);
                    } else {
                        unreachable!("i think?")
                    }
                }

                CF::Normal => {
                    let next = node.index() + 1;
                    if next < stmts.len() {
                        graph.add_edge(node, (next as u32).into(), false);
                    }
                }

                CF::Return => {
                    //do nothing
                }
            }
        }
        Self { graph, defs }
    }

    /// returns entry point of CFG, assumes that the graph is nonempty
    pub fn start(&self) -> NodeIndex {
        self.graph.node_indices().next().unwrap()
    }

    pub fn recompute_defs(&mut self) {
        let mut defs = HashMap::<Var, HashSet<NodeIndex>>::new();
        for node in self.graph.node_indices() {
            //build defs table
            for var in self.graph.node_weight(node).unwrap().defs() {
                match defs.get_mut(&var) {
                    Some(set) => {
                        set.insert(node.clone());
                    }
                    None => {
                        defs.insert(var, HashSet::from([node]));
                    }
                };
            }
        }
        self.defs = defs;
    }

    pub fn def(&self, var: &Var) -> HashSet<NodeIndex> {
        match self.defs.get(var) {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }

    pub fn wrap_SSA(&self) -> CFG<SSANode<T>> {
        let mut graph = Graph::<SSANode<T>, bool>::new();
        for node in self.graph.node_indices() {
            graph.add_node(SSANode::new(self.graph.node_weight(node).unwrap().clone()));
        }
        for edge in self.graph.edge_indices() {
            let (a, b) = self.graph.edge_endpoints(edge).unwrap();
            let weight = self.graph.edge_weight(edge).unwrap();
            graph.add_edge(a, b, *weight);
        }

        CFG {
            graph,
            defs: self.defs.clone(),
        }
    }

    pub fn reverse_postorder(&self) -> Vec<NodeIndex> {
        let mut visited = HashSet::new();
        let mut postorder = Vec::new();
        let mut work_list = Vec::new();
        work_list.push(self.start());
        while let Some(node) = work_list.pop() {
            if !visited.contains(&node) {
                visited.insert(node);
                work_list.push(node);
                for neighbor in self.graph.neighbors(node) {
                    if !visited.contains(&neighbor) {
                        work_list.push(neighbor);
                    }
                }
            } else {
                postorder.push(node);
            }
        }
        postorder.reverse();
        postorder
    }

    pub fn work_list<U: PartialEq + Clone + std::fmt::Debug>(
        &self,
        meet: fn(&U, &U) -> U,                    //meet function
        transfer: fn(&U, &NodeIndex, &Self) -> U, //F[n]
        top: U,                                   //initial value of out[n]
        init: U,                                  //initial value of meet (usually empty set)
        direction: Dir, //direction of analysis, Dir::Forward or Dir::Backward
        return_outs: bool,
    ) -> Vec<U> {
        let mut work_list = VecDeque::new();
        let mut work_list_set = HashSet::new();
        let mut out_n = vec![top.clone(); self.graph.node_count()];
        let mut in_n = vec![top.clone(); self.graph.node_count()];
        let mut visited = Vec::new();
        let direction = match direction {
            Dir::Forward => petgraph::Direction::Incoming,
            Dir::Backward => petgraph::Direction::Outgoing,
        };

        let reverse_dir = match direction {
            petgraph::Direction::Incoming => petgraph::Direction::Outgoing,
            petgraph::Direction::Outgoing => petgraph::Direction::Incoming,
        };

        for node in self.graph.node_indices() {
            work_list.push_back(node);
            work_list_set.insert(node);
        }

        while !work_list.is_empty() {
            let node = work_list.pop_front().unwrap();
            work_list_set.remove(&node);
            //meet of pred/succ nodes (depending on direction), assumes that init is empty set
            match direction {
                //Backward analysis, meet of successors
                petgraph::Direction::Outgoing => {
                    let out = self
                        .graph
                        .neighbors_directed(node, direction)
                        .fold(init.clone(), |acc, x| meet(&acc, &in_n[x.index()]));
                    let new_result = transfer(&out, &node, self);
                    out_n[node.index()] = out;
                    //if result changed, add to worklist
                    if new_result != in_n[node.index()] {
                        in_n[node.index()] = new_result;
                        visited.push(node);
                        for neighbor in self.graph.neighbors_directed(node, reverse_dir) {
                            if work_list_set.insert(neighbor) {
                                work_list.push_back(neighbor);
                            }
                        }
                    }
                }
                //Forward analysis, meet of predecessors
                petgraph::Direction::Incoming => {
                    let in_ = self
                        .graph
                        .neighbors_directed(node, direction)
                        .fold(init.clone(), |acc, x| meet(&acc, &out_n[x.index()]));
                    let new_result = transfer(&in_, &node, self);
                    in_n[node.index()] = in_;
                    //if result changed, add to worklist
                    if new_result != out_n[node.index()] {
                        out_n[node.index()] = new_result;
                        visited.push(node);
                        for neighbor in self.graph.neighbors_directed(node, reverse_dir) {
                            if work_list_set.insert(neighbor) {
                                work_list.push_back(neighbor);
                            }
                        }
                    }
                }
            };
        }

        if return_outs {
            out_n
        } else {
            in_n
        }
    }

    pub fn delete_unreachable(&mut self) {
        let reachable = self.reachable_from_start();
        self.graph.retain_nodes(|_, node| reachable.contains(&node));
        self.recompute_defs();
    }

    pub fn reachable_from_start(&self) -> HashSet<NodeIndex> {
        let mut visited = HashSet::new();
        self.reachable_helper(&self.start(), &mut visited);
        visited
    }

    fn reachable_helper(&self, node: &NodeIndex, visited: &mut HashSet<NodeIndex>) {
        visited.insert(*node);
        for neighbor in self.graph.neighbors(*node) {
            if !visited.contains(&neighbor) {
                self.reachable_helper(&neighbor, visited);
            }
        }
    }
    pub fn debug_cfg(&self) {
        println!("{:?}", Dot::with_config(&self.graph, &[]));
    }

    pub fn debug_cfg_string(&self) -> String {
        let mut out = String::new();
        writeln!(out, "{:?}", Dot::with_config(&self.graph, &[]));
        out
    }
}

impl CFGNode for LIRNode {
    fn uses(&self) -> HashSet<Var> {
        match self {
            LIRNode::Move { val: (l, r), .. } => match &**l {
                LIRNode::Name { .. } | LIRNode::Temp { .. } => r.uses(),
                _ => vec![l.uses(), r.uses()].into_iter().flatten().collect(),
            },
            LIRNode::Call {
                val: (_, nodes, _), ..
            } => nodes.into_iter().map(|x| x.uses()).flatten().collect(),
            LIRNode::Jump { .. } => HashSet::new(),
            LIRNode::CJump { val: (cond, _), .. } => cond.uses(),
            LIRNode::Return { val, .. } => val.into_iter().map(|x| x.uses()).flatten().collect(),
            LIRNode::Label { .. } => HashSet::new(),
            LIRNode::Const { .. } => HashSet::new(),
            LIRNode::Temp { val, .. } => HashSet::from([Var::Name(val.clone())]),
            LIRNode::Op { val: (_, l, r), .. } => {
                vec![l.uses(), r.uses()].into_iter().flatten().collect()
            }
            LIRNode::Mem { val, .. } => val.uses(),
            LIRNode::Name { val, .. } => HashSet::from([Var::Name(val.clone())]),
            LIRNode::SSA_Temp { val: (temp, n), .. } => {
                HashSet::from([Var::Name(format!("{}_{}", temp.clone(), *n))])
            }
        }
    }

    fn defs(&self) -> HashSet<Var> {
        match self {
            LIRNode::Move { val: (l, _), .. } => l.defs(),
            LIRNode::Name { val, .. } | LIRNode::Temp { val, .. } => {
                HashSet::from([Var::Name(val.clone())])
            }
            _ => HashSet::new(),
        }
    }

    fn is_label(&self) -> Option<String> {
        match self {
            LIRNode::Label { val, .. } => Some(val.clone()),
            _ => None,
        }
    }

    fn control_flow(&self) -> CF {
        match self {
            LIRNode::Jump { val, .. } => CF::Jump(val.clone()),
            LIRNode::CJump {
                val: (_, label), ..
            } => CF::CJump(label.clone()),
            LIRNode::Return { .. } => CF::Return,
            _ => CF::Normal,
        }
    }
}

impl CFGNode for ins {
    fn uses(&self) -> HashSet<Var> {
        match self {
            (OPCODE::call(args), _, _) => match args {
                0 => HashSet::new(),
                1 => HashSet::from([Var::Reg(Register::RDI)]),
                _ => HashSet::from([Var::Reg(Register::RDI), Var::Reg(Register::RSI)]),
            },
            (OPCODE::lea, _dest, src) => src.to_var(),
            (OPCODE::mov, dest, src) => {
                if dest.is_deref() {
                    vec![dest.to_var(), src.to_var()]
                        .into_iter()
                        .flatten()
                        .collect()
                } else {
                    src.to_var()
                }
            }
            (opcode, dest, src) => {
                if opcode.is_set() {
                    if dest.is_deref() {
                        dest.to_var()
                    } else {
                        HashSet::new()
                    }
                } else {
                    vec![dest.to_var(), src.to_var()]
                        .into_iter()
                        .flatten()
                        .collect()
                }
            }
        }
    }

    fn defs(&self) -> HashSet<Var> {
        match self {
            (OPCODE::call(_), _, _) => {
                HashSet::from([Var::Reg(Register::RAX), Var::Reg(Register::RDX)])
            }

            (_, dest, _src) => match dest {
                loc::Global(_) | loc::Register(_) => dest.to_var(),
                _ => HashSet::new(),
            },
        }
    }

    fn is_label(&self) -> Option<String> {
        match self {
            (OPCODE::LABEL, dest, _src) => Some(dest.label_as_string()),
            _ => None,
        }
    }

    fn control_flow(&self) -> CF {
        match self {
            (OPCODE::jmp, dest, _) => CF::Jump(dest.label_as_string()),
            (OPCODE::jz, dest, _)
            | (OPCODE::je, dest, _)
            | (OPCODE::jnz, dest, _)
            | (OPCODE::jne, dest, _)
            | (OPCODE::jl, dest, _)
            | (OPCODE::jle, dest, _)
            | (OPCODE::jg, dest, _)
            | (OPCODE::jge, dest, _)
            | (OPCODE::jb, dest, _)
            | (OPCODE::jbe, dest, _)
            | (OPCODE::ja, dest, _)
            | (OPCODE::jae, dest, _) => CF::CJump(dest.label_as_string()),
            (OPCODE::ret, _, _) => CF::Return,
            _ => CF::Normal,
        }
    }
}

//basic blocks
impl<T: CFGNode> CFGNode for Vec<T> {
    fn uses(&self) -> HashSet<Var> {
        self.iter().map(|x| x.uses()).flatten().collect()
    }

    fn defs(&self) -> HashSet<Var> {
        self.iter().map(|x| x.defs()).flatten().collect()
    }

    fn is_label(&self) -> Option<String> {
        self.first().unwrap().is_label()
    }

    fn control_flow(&self) -> CF {
        match self.last() {
            Some(x) => x.control_flow(),
            None => CF::Normal,
        }
    }
}

fn basic_blocks<T: CFGNode>(stmts: Vec<T>) -> Vec<Vec<T>> {
    let mut blocks = Vec::new();
    let mut block = Vec::new();
    for stmt in stmts {
        match stmt.control_flow() {
            CF::Jump(_) | CF::CJump(_) | CF::Return => {
                block.push(stmt);
                blocks.push(block);
                block = Vec::new();
            }
            CF::Normal => {
                if stmt.is_label().is_some() {
                    blocks.push(block);
                    block = Vec::new();
                }
                block.push(stmt);
            }
        }
    }
    blocks.push(block);
    blocks.into_iter().filter(|x| !x.is_empty()).collect() //remove empty
}

pub fn graph_from_bblocks<T: CFGNode + Clone + std::fmt::Debug>(stmts: Vec<T>) -> CFG<Vec<T>> {
    CFG::new(&basic_blocks(stmts))
}

//trace finding
impl CFG<Vec<LIRNode>> {
    /// performs trace analysis and block reordering to get a vector of LIRNodes
    /// returns a vector of LIRNodes and the size of the resulting LIRTree
    pub fn flatten(&mut self) -> (Vec<LIRNode>, usize) {
        let mut marked = HashSet::new();
        let mut counts = HashMap::new();
        let mut traces = vec![];

        let mut removed_edges = vec![];
        for edge in self.graph.edge_indices() {
            if *self.graph.edge_weight(edge).unwrap() {
                removed_edges.push(self.graph.edge_endpoints(edge).unwrap());
            }
        }

        for edge in &removed_edges {
            self.graph
                .remove_edge(self.graph.find_edge(edge.0, edge.1).unwrap());
        }

        loop {
            if let Some(source) = Self::heuristic(self.graph.node_indices(), &marked, &counts) {
                let mut memo = HashMap::new();
                Self::find_maximal_trace(&mut marked, &mut memo, source, &self.graph);
                let trace = Self::reconstruct_trace(&memo, source);
                Self::update_counts(&trace, &memo, &mut counts);
                for block in trace.iter() {
                    marked.insert(*block);
                }
                traces.push(trace);
            } else {
                break;
            }
        }

        let jumps_to_insert = Self::reorder_traces(&mut traces, &self.graph);
        let code = traces
            .into_iter()
            .enumerate()
            .map(|(idx, x)| {
                let mut extract_node_weights = x
                    .into_iter()
                    .map(|x| self.graph.node_weight(x).unwrap().clone())
                    .collect::<Vec<_>>();

                for i in 0..extract_node_weights.len() - 1 {
                    let x = extract_node_weights.get_mut(i).unwrap();
                    if let CF::Jump(_) = x.control_flow() {
                        x.pop();
                    }
                }
                let mut conv_lir = extract_node_weights
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();

                if let Some(jtarget) = jumps_to_insert[idx] {
                    //if the jump is already there, no need to add
                    if let CF::Jump(_) = conv_lir.control_flow() {
                        ();
                    } else {
                        let target = self
                            .graph
                            .node_weight_mut(NodeIndex::new(jtarget as usize))
                            .unwrap();
                        match target.is_label() {
                            Some(t) => conv_lir.push(LIRNode::Jump { idx: 0, val: t }),
                            None => {
                                panic!();
                                // let new_label = format!("__flattenL{}_{}", jtarget, next_int());
                                // conv_lir.push(LIRNode::Jump {
                                //     idx: 0,
                                //     val: new_label.clone(),
                                // });
                                // target.insert(
                                //     0,
                                //     LIRNode::Label {
                                //         idx: 0,
                                //         val: new_label,
                                //     },
                                // );
                            }
                        }
                    }
                } else {
                    //if there is a jump, remove it (extraneous)
                    if let CF::Jump(_) = conv_lir.control_flow() {
                        conv_lir.pop();
                    }
                }
                conv_lir
            })
            .flatten()
            .collect::<Vec<LIRNode>>();

        //remove extra labels
        let used_labels = code
            .iter()
            .map(|x| match x.control_flow() {
                CF::Jump(l) | CF::CJump(l) => Some(l),
                _ => None,
            })
            .flatten()
            .collect::<HashSet<_>>();
        let mut removed_extra_labels = code;
        // let mut removed_extra_labels = code
        //     .into_iter()
        //     .filter(|x| {
        //         if let LIRNode::Label { val, .. } = x {
        //             used_labels.contains(val)
        //         } else {
        //             true
        //         }
        //     })
        //     .collect::<Vec<_>>();
        //restore removed edges
        for edge in removed_edges {
            self.graph.add_edge(edge.0, edge.1, true);
        }
        let size = renumber_lir(&mut removed_extra_labels);
        (removed_extra_labels, size)
    }

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

    fn find_maximal_trace<T>(
        marked: &mut HashSet<NodeIndex>,
        memo: &mut HashMap<NodeIndex, (i32, Option<NodeIndex>)>,
        source: NodeIndex,
        cfg: &Graph<T, bool>,
    ) -> i32 {
        marked.insert(source);
        let entry = match cfg
            .neighbors(source)
            .map(|x| {
                if !marked.contains(&x) {
                    match memo.get(&x) {
                        Some((trace_len, _)) => (trace_len.clone(), x),
                        None => (Self::find_maximal_trace(marked, memo, x, cfg), x),
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

    fn reorder_traces<T>(
        trace_list: &mut Vec<Vec<NodeIndex>>,
        graph: &Graph<T, bool>,
    ) -> Vec<Option<isize>> {
        //move first trace to start
        let start = (0..trace_list.len())
            .min_by_key(|x| trace_list[*x].first().unwrap().index())
            .unwrap();
        trace_list.swap(0, start);

        let mut trace_starting_with = trace_list
            .iter()
            .enumerate()
            .map(|(idx, x)| (x.first().unwrap().index(), idx))
            .collect::<HashMap<_, _>>();

        let mut inserted_jumps = vec![None; trace_list.len()];
        for i in 0..trace_list.len() {
            let last = trace_list[i].last().unwrap().index();
            let next = graph.edges(NodeIndex::new(last)).next();
            match next {
                Some(edge) => {
                    assert!(*edge.weight() == false);
                    let target = edge.target().index();
                    let target_idx = trace_starting_with.get(&target);
                    match target_idx {
                        Some(idx) => {
                            if *idx > i {
                                trace_list.swap(i, *idx);
                                trace_starting_with
                                    .insert(trace_list[*idx].first().unwrap().index(), *idx);
                            } else {
                                inserted_jumps[i] = Some(target as isize);
                            }
                        }
                        None => {
                            inserted_jumps[i] = Some(target as isize);
                        }
                    }
                }
                None => inserted_jumps[i] = None,
            }
        }
        inserted_jumps
    }
}

impl<T: CFGNode + Clone + std::fmt::Debug> CFG<SSANode<T>> {
    pub fn unwrap_SSA(&self) -> CFG<T> {
        let mut graph = Graph::<T, bool>::new();
        for node in self.graph.node_indices() {
            graph.add_node(self.graph.node_weight(node).unwrap().unwrap());
        }
        for edge in self.graph.edge_indices() {
            let (a, b) = self.graph.edge_endpoints(edge).unwrap();
            let weight = self.graph.edge_weight(edge).unwrap();
            graph.add_edge(a, b, *weight);
        }

        CFG {
            graph,
            defs: self.defs.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SSANode<T: CFGNode + Clone + std::fmt::Debug> {
    pub phi: HashMap<String, usize>,
    pub phi_uses: HashMap<String, Vec<usize>>,
    pub val: T,
    pub last_def: HashMap<String, usize>,
}

impl<T: CFGNode + Clone + std::fmt::Debug> SSANode<T> {
    pub fn new(val: T) -> Self {
        Self {
            phi: HashMap::new(),
            phi_uses: HashMap::new(),
            val,
            last_def: HashMap::new(),
        }
    }

    pub fn unwrap(&self) -> T {
        self.val.clone()
    }

    pub fn update_phi(&mut self) -> Vec<(String, usize)> {
        let mut new_phi = Vec::new();
        let keys = self.phi.keys().cloned().collect::<Vec<_>>();
        for var in keys {
            let curr = self.phi.get(&var).unwrap();
            if *curr == 0 {
                let next = next_int();
                self.phi.insert(var.clone(), next);
                new_phi.push((var.clone(), next));
            } else {
                new_phi.push((var.clone(), *curr));
            }
        }
        new_phi
    }
}

impl<T: CFGNode + Clone + std::fmt::Debug> CFGNode for SSANode<T> {
    fn control_flow(&self) -> CF {
        self.val.control_flow()
    }

    fn defs(&self) -> HashSet<Var> {
        let additional_phi_defs = self
            .phi
            .keys()
            .cloned()
            .map(|x| Var::Name(x))
            .collect::<HashSet<_>>();
        //union self.val.defs() and additional_phi_defs
        self.val
            .defs()
            .union(&additional_phi_defs)
            .cloned()
            .collect()
    }

    fn uses(&self) -> HashSet<Var> {
        self.val.uses()
    }

    fn is_label(&self) -> Option<String> {
        self.val.is_label()
    }
}

impl<T: CFGNode + Clone + std::fmt::Debug> std::fmt::Display for SSANode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.defs())
    }
}
