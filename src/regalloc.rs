use crate::{
    analysis::live_variable_analysis,
    cfg::{graph_from_bblocks, CFGNode, Var, CFG},
    instructions, next_int,
    translate_types::{ins, loc, Register, OPCODE},
};
use petgraph::graph::NodeIndex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

const CALLER_SAVED: [Register; 6] = [
    // Register::RAX,
    // Register::RCX,
    // Register::RDX,
    Register::RSI,
    Register::RDI,
    Register::R8,
    Register::R9,
    Register::R10,
    Register::R11,
];

const CALLEE_SAVED: [Register; 5] = [
    Register::RBX,
    Register::R12,
    Register::R13,
    Register::R14,
    Register::R15,
];

fn next_spill_register() -> loc {
    match next_int() % 3 {
        0 => loc::Register(Register::RAX),
        1 => loc::Register(Register::RCX),
        2 => loc::Register(Register::RDX),
        _ => unreachable!(),
    }
}

fn reg_loc_helper(
    register: &Register,
    register_map: &HashMap<Var, loc>,
    moved_spill: &mut HashMap<Var, loc>,
) -> (Vec<ins>, loc) {
    if register.is_named() {
        (vec![], loc::Register(register.clone()))
    } else {
        //println!("register: {:?}", register);
        match moved_spill.get(&Var::Reg(register.clone())) {
            Some(preassigned_loc) => (vec![], preassigned_loc.clone()),
            None => {
                let loc = register_map.get(&Var::Reg(register.clone())).unwrap();
                if loc.is_register() {
                    (vec![], loc.clone())
                } else {
                    let next_spill = next_spill_register();
                    moved_spill.insert(Var::Reg(register.clone()), next_spill.clone());
                    (
                        vec![(OPCODE::mov, next_spill.clone(), loc.clone())],
                        next_spill,
                    )
                }
            }
        }
    }
}
//produces the vector of instructions to move the appropriate value into loc
fn loc_helper(
    loc: &loc,
    register_map: &HashMap<Var, loc>,
    moved_spill: &mut HashMap<Var, loc>,
    emit_ins: bool,
) -> (Vec<ins>, loc) {
    match loc {
        loc::Register(register) => {
            if emit_ins {
                reg_loc_helper(register, register_map, moved_spill)
            } else {
                (
                    vec![],
                    reg_loc_helper(register, register_map, moved_spill).1,
                )
            }
        }
        loc::Deref(a, b, c, d) => {
            let mut ins = vec![];
            let a_loc = if a.is_some() {
                let (mut a_ins, a_loc) = reg_loc_helper(&a.unwrap(), register_map, moved_spill);
                ins.append(&mut a_ins);
                Some(a_loc.unwrap_reg())
            } else {
                None
            };
            let b_loc = if b.is_some() {
                let (mut b_ins, b_loc) = reg_loc_helper(&b.unwrap(), register_map, moved_spill);
                ins.append(&mut b_ins);
                Some(b_loc.unwrap_reg())
            } else {
                None
            };
            if emit_ins {
                (ins, loc::Deref(a_loc, b_loc, *c, *d))
            } else {
                (vec![], loc::Deref(a_loc, b_loc, *c, *d))
            }
        }
        _ => (vec![], loc.clone()),
    }
}

pub fn replace_registers(register_map: &HashMap<Var, loc>, instruction: ins) -> Vec<ins> {
    let (opcode, dest, src) = instruction;
    match (src.is_imm32(), &dest, &src) {
        (false, loc::Deref(_, _, _, _), loc::Literal(_))
        | (false, loc::Global(_), loc::Literal(_)) => {
            let mut seq = vec![];
            let mut register_names = HashMap::new();
            let (mut dest_ins, new_dest) = loc_helper(
                &dest,
                register_map,
                &mut register_names,
                opcode.reads_dest(),
            );
            seq.append(&mut dest_ins);
            let imm_temp = next_spill_register();
            seq.push((OPCODE::mov, imm_temp.clone(), src));
            seq.push((opcode, new_dest, imm_temp));
            return seq;
        }
        (false, loc::Register(r), loc::Literal(_)) => {
            let mut seq = vec![];
            let imm_temp = next_spill_register();
            seq.push((OPCODE::mov, imm_temp.clone(), src));
            match register_map.get(&Var::Reg(r.clone())) {
                Some(stack_loc) => seq.push((OPCODE::mov, stack_loc.clone(), imm_temp)),
                None => seq.push((OPCODE::mov, dest, imm_temp)),
            };
            return seq;
        }
        _ => (),
    };
    match (src.is_deref(), dest.is_deref()) {
        (true, true) => unreachable!(),
        (true, false) | (false, true) => {
            let mut seq = vec![];
            let mut register_names = HashMap::new();
            let (mut dest_ins, new_dest) = loc_helper(
                &dest,
                register_map,
                &mut register_names,
                opcode.reads_dest() || dest.is_deref(),
            );
            seq.append(&mut dest_ins);
            let (mut src_ins, new_src) = loc_helper(&src, register_map, &mut register_names, true);
            seq.append(&mut src_ins);
            if opcode.is_set() {
                seq.push((opcode, new_dest.lower_byte(), new_src)); //set register
                seq.push((OPCODE::movsx, new_dest.clone(), new_dest.lower_byte()));
                //sign extend
            } else {
                seq.push((opcode, new_dest.clone(), new_src));
            }
            if opcode.is_set()
                || (opcode.mutates_dest()
                    && !dest.is_deref()
                    && register_map
                        .get(&Var::Reg(dest.unwrap_reg()))
                        .unwrap()
                        .is_deref())
            {
                //need to push result back onto the stack in this case
                match register_map.get(&Var::Reg(dest.unwrap_reg())) {
                    Some(stack_pos) => seq.push((OPCODE::mov, stack_pos.clone(), new_dest)),
                    None => (),
                }
            };
            seq
        }
        (false, false) => {
            if opcode.is_set() {
                let mut seq = vec![];
                let mut register_names = HashMap::new();
                let (mut dest_ins, new_dest) = loc_helper(
                    &dest,
                    register_map,
                    &mut register_names,
                    opcode.reads_dest(),
                );
                seq.append(&mut dest_ins);
                let (mut src_ins, new_src) =
                    loc_helper(&src, register_map, &mut register_names, true);
                seq.append(&mut src_ins);
                seq.push((opcode, new_dest.lower_byte(), new_src)); //set register
                seq.push((OPCODE::movsx, new_dest.clone(), new_dest.lower_byte()));
                if register_map
                    .get(&Var::Reg(dest.unwrap_reg()))
                    .unwrap()
                    .is_deref()
                {
                    seq.push((
                        OPCODE::mov,
                        register_map
                            .get(&Var::Reg(dest.unwrap_reg()))
                            .unwrap()
                            .clone(),
                        new_dest,
                    ));
                }
                //sign extend
                seq
            } else {
                let mut seq = vec![];
                let mut register_names = HashMap::new();
                let new_dest = if dest.is_named_reg() {
                    dest
                } else if dest.is_register() {
                    match register_map.get(&Var::Reg(dest.unwrap_reg())) {
                        Some(loc) => loc.clone(),

                        None => unreachable!(
                            "register {:?} not in register map {:?} ",
                            dest, register_map
                        ),
                    }
                } else {
                    dest
                };
                let (mut src_ins, new_src) =
                    loc_helper(&src, register_map, &mut register_names, true);
                seq.append(&mut src_ins);
                seq.push((opcode, new_dest, new_src));
                seq
            }
        }
    }
}

fn fix_double_mem_operand(instruction: ins) -> Vec<ins> {
    match instruction {
        (OPCODE::lea, loc::Deref(_, _, _, _), loc::Deref(_, _, _, _)) => {
            let temp = loc::temp();
            vec![
                (OPCODE::mov, temp.clone(), instruction.1),
                (instruction.0, temp, instruction.2),
            ]
        }

        (_, loc::Deref(_, _, _, _), loc::Deref(_, _, _, _)) => {
            let temp = loc::temp();
            vec![
                (OPCODE::mov, temp.clone(), instruction.2),
                (instruction.0, instruction.1, temp),
            ]
        }
        _ => vec![instruction],
    }
}

fn contains_dead_registers(ins: &ins, register_map: &HashMap<Var, loc>) -> bool {
    let (_, dest, src) = ins;
    match (dest, src) {
        (loc::Register(r), _) => !(register_map.contains_key(&Var::Reg(r.clone())) || r.is_named()),
        _ => false,
    }
}

fn is_redundant(ins: &ins) -> bool {
    match ins {
        //moves from a register to the same register
        (OPCODE::mov, loc::Register(r1), loc::Register(r2)) => r1 == r2,
        _ => false,
    }
}

pub fn alloc_register_linear(stmts: Vec<ins>, max_fn_args: usize) -> Vec<ins> {
    let stmts = stmts
        .into_iter()
        .map(fix_double_mem_operand)
        .flatten()
        .collect::<Vec<_>>();
    let (register_map, spilled) = linear_scan(&stmts);
    //write prologue based on stack size and used callee-saved registers
    let mut prologue = vec![];

    //println!("register map: {:?}", register_map);
    let used_registers = register_map
        .iter()
        .filter_map(|x| match x.1 {
            loc::Register(r) => {
                if CALLEE_SAVED.contains(&r) {
                    Some(r)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut saved_reg_offset = spilled * 8;
    let mut stack_size = spilled + used_registers.len() as isize + max_fn_args as isize;
    if stack_size % 2 == 1 {
        stack_size += 1;
    }
    //push rbp onto stack
    prologue.push((OPCODE::push, loc::rbp(), loc::Null));
    //set rbp to rsp
    prologue.push((OPCODE::mov, loc::rbp(), loc::rsp()));
    //sub rsp by stack_size
    if stack_size > 0 {
        prologue.push((OPCODE::sub, loc::rsp(), loc::Literal(stack_size * 8)));
    }

    //push used registers onto stack
    for reg in used_registers.iter() {
        saved_reg_offset += 8;
        prologue.push((
            OPCODE::mov,
            loc::Deref(
                None,
                Some(loc::rbp().unwrap_reg()),
                None,
                Some(-saved_reg_offset),
            ),
            loc::Register(*reg.clone()),
        ));
    }

    let mut epilogue = vec![];
    saved_reg_offset = spilled * 8;
    //pop used registers off stack
    for reg in used_registers.iter() {
        saved_reg_offset += 8;
        epilogue.push((
            OPCODE::mov,
            loc::Register(*reg.clone()),
            loc::Deref(
                None,
                Some(loc::rbp().unwrap_reg()),
                None,
                Some(-saved_reg_offset),
            ),
        ));
    }
    epilogue.push((OPCODE::leave, loc::null(), loc::null()));

    //translate fn body
    let out: Vec<ins> = prologue
        .into_iter()
        .chain(
            //remove dead registers, fix double mem operands, and replace registers
            stmts
                .into_iter()
                .filter(|ins| !contains_dead_registers(ins, &register_map))
                .flat_map(|ins| replace_registers(&register_map, ins)),
        )
        //replace epilogue markers
        .flat_map(|x| {
            if let (OPCODE::EPILOGUE, loc::Null, loc::Null) = x {
                epilogue.clone()
            } else {
                vec![x]
            }
        })
        .filter(|ins| !is_redundant(ins))
        .collect();
    // for ins in &out {
    //     println!("{:?}", ins);
    // }
    out
}
//performs a linear scan register allocation using live variable analysis
fn linear_scan(stmts: &Vec<ins>) -> (HashMap<Var, loc>, isize) {
    let cfg = CFG::new(&stmts);
    let live_vars = live_variable_analysis(&cfg);

    // println!("live vars");
    // for (i, live_vars) in live_vars.iter().enumerate() {
    //     println!("{}: {:?}", i, live_vars);
    // }
    // for node in cfg.graph.node_indices() {
    //     println!(
    //         "{:?}, {:?}",
    //         node,
    //         cfg.graph.node_weight(node).unwrap().defs()
    //     );
    // }
    //find intervals
    let mut named_reg_intervals = HashMap::<Register, Vec<(i32, i32)>>::new();
    let mut intervals: HashMap<Var, (i32, i32)> = HashMap::new();
    for (i, live_vars) in live_vars.iter().enumerate() {
        for var in live_vars {
            if var.is_named_reg() {
                named_reg_intervals
                    .entry(var.unwrap_reg())
                    .or_insert(vec![])
                    .push((i as i32, i as i32));
            } else {
                if !intervals.contains_key(var) {
                    intervals.insert(var.clone(), (i as i32, i as i32));
                } else {
                    let interval = intervals.get_mut(var).unwrap();
                    interval.1 = i as i32;
                }
            }
        }
    }
    // println!("itervals");
    // for interval in &intervals {
    //     println!("{:?}", interval);
    // }
    //find all nodes that call a function
    let calls = cfg
        .graph
        .node_indices()
        .filter_map(|x| {
            let node_weight = cfg.graph.node_weight(x).unwrap();
            match node_weight {
                (OPCODE::call(_), _, _) => Some(x.index() as i32),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    let mut sorted_intervals = intervals.into_iter().collect::<Vec<(Var, (i32, i32))>>();
    sorted_intervals.sort_unstable_by(|a, b| a.1 .0.cmp(&b.1 .0));

    // println!(
    //     "interval analysis: {:?}",
    //     analyze_intervals(&sorted_intervals)
    // );

    //min heap denoting the next expiring interval
    let mut intervals_expiration = BinaryHeap::<Reverse<(i32, loc)>>::new();
    let mut active: HashSet<Register> = HashSet::new();
    let mut available_stack: HashSet<loc> = HashSet::new();
    let mut assigned_registers = HashMap::<Var, loc>::new();
    let mut spilled_variables = 0;
    for (var, (start, end)) in sorted_intervals {
        //remove expired intervals
        let mut just_expired = vec![];
        let mut just_expired_stack = None;
        while !intervals_expiration.is_empty() && intervals_expiration.peek().unwrap().0 .0 < start
        {
            let Reverse((_, loc)) = intervals_expiration.pop().unwrap();
            match loc {
                loc::Register(register) => {
                    active.remove(&register);
                    just_expired.push(register);
                }
                loc::Deref(_, _, _, _) => {
                    available_stack.insert(loc.clone());
                    just_expired_stack = Some(loc);
                }
                _ => {}
            }
        }

        //if it is a hardcoded reg, add to active set
        if var.is_named_reg() {
            active.insert(var.unwrap_reg());
            intervals_expiration.push(Reverse((end, loc::Register(var.unwrap_reg()))));
            continue;
        }
        let just_expired = match just_expired.len() {
            1 => Some(just_expired[0]),
            _ => None,
        };

        let register = assign_register(
            (var.clone(), (start, end)),
            &active,
            &named_reg_intervals,
            &calls,
            just_expired,
        );

        //if there are no available registers, spill the current var
        if register.is_none() {
            if available_stack.is_empty() {
                spilled_variables += 1;
                assigned_registers.insert(
                    var,
                    loc::Deref(
                        None,
                        Some(Register::RBP),
                        None,
                        Some(-(spilled_variables * 8)),
                    ),
                );
                intervals_expiration.push(Reverse((
                    end,
                    loc::Deref(
                        None,
                        Some(Register::RBP),
                        None,
                        Some(-(spilled_variables * 8)),
                    ),
                )));
            } else {
                let loc = match just_expired_stack {
                    Some(loc) => loc,
                    None => available_stack.iter().next().unwrap().clone(),
                };

                available_stack.remove(&loc);
                assigned_registers.insert(var, loc.clone());
                intervals_expiration.push(Reverse((end, loc)));
            }
        } else {
            let register = register.unwrap();
            assigned_registers.insert(var, loc::Register(register.clone()));
            active.insert(register.clone());

            intervals_expiration.push(Reverse((end, loc::Register(register.clone()))));
        }
    }
    // for entry in &assigned_registers {
    //     println!("{:?}", entry);
    // }
    //println!("{:?}", assigned_registers);
    (assigned_registers, spilled_variables as isize)
}

fn assign_register(
    interval: (Var, (i32, i32)),
    active: &HashSet<Register>,
    named_reg_intervals: &HashMap<Register, Vec<(i32, i32)>>,
    calls: &Vec<i32>,
    just_expired: Option<Register>,
) -> Option<Register> {
    //find available registers
    let available_caller_saved_reg = CALLER_SAVED
        .iter()
        .filter(|x| !active.contains(x))
        .collect::<Vec<_>>();
    let available_callee_saved_reg = CALLEE_SAVED
        .iter()
        .filter(|x| !active.contains(x))
        .collect::<Vec<_>>();

    //if there are no available registers, spill the interval with the latest end point
    if available_caller_saved_reg.is_empty() && available_callee_saved_reg.is_empty() {
        return None;
    }

    //if there are available registers, check if the current interval overlaps a function call
    //use binary search to find the first call greater than the start of the interval
    let overlaps_call = if calls.is_empty() {
        false
    } else {
        let mut left = 0;
        let mut right = calls.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if calls[mid] >= interval.1 .0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        calls[left] >= interval.1 .0 && calls[left] <= interval.1 .1
    };
    //if it does, assign it a callee saved registers, if none are available, spill (return None)
    if overlaps_call {
        if available_callee_saved_reg.is_empty() {
            return None;
        } else {
            if just_expired.is_some() && just_expired.unwrap().is_callee_saved() {
                return just_expired;
            }
            return Some(*available_callee_saved_reg[0]);
        }
    }

    //if it doesn't, assign it the callee saved register
    //if it does not overlap a function call, give it any register, but prioritize caller saved registers.
    //do not give it a callee saved register if the current live interval goes over a live interval of a named_register in named_reg_intervals
    fn overlaps_live(
        register: &Register,
        interval: &(i32, i32),
        named_reg_intervals: &HashMap<Register, Vec<(i32, i32)>>,
    ) -> bool {
        match named_reg_intervals.get(register) {
            Some(intervals) => {
                for (start, end) in intervals {
                    if interval.0 <= *end && interval.1 >= *start {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    if just_expired.is_some() {
        if !overlaps_live(&just_expired.unwrap(), &interval.1, &named_reg_intervals) {
            return just_expired;
        }
    }

    for reg in available_caller_saved_reg {
        if !overlaps_live(reg, &interval.1, &named_reg_intervals) {
            return Some(*reg);
        }
    }
    //no caller saved registers available, assign a callee saved register if one is available, spill otherwise
    if available_callee_saved_reg.is_empty() {
        return None;
    } else {
        return Some(*available_callee_saved_reg[0]);
    }
}

/// debug function to analyze overlapping intervals
/// given list of interval return max overlapping intervals
fn analyze_intervals(intervals: &Vec<(Var, (i32, i32))>) -> usize {
    let min = intervals.iter().map(|x| x.1 .0).min().unwrap();
    let max = intervals.iter().map(|x| x.1 .1).max().unwrap();

    let mut overlaps = vec![0; (max - min + 1) as usize];
    for (_, (start, end)) in intervals {
        for i in *start..=*end {
            overlaps[(i - min) as usize] += 1;
        }
    }
    *overlaps.iter().max().unwrap()
}
