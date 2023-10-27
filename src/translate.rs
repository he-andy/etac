#![allow(non_camel_case_types)]

use crate::cfg::CFG;
use crate::ir_types::{LIRCompUnit, LIRFuncDecl, LIRStmt};
use crate::translate_types::{
    ins, ins_to_string, loc, LIRNode, Register, TTable, Translation, OPCODE,
};
use crate::{cfg, next_int, regalloc, reset, ssa};
use crate::{copyprop, dce, loop_opt};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct OptOptions {
    pub cf: bool,
    pub reg: bool,
    pub copy: bool,
    pub dce: bool,
    pub inl: bool,
}

fn next_reg() -> loc {
    match next_int() % 3 {
        0 => loc::rax(),
        1 => loc::rcx(),
        2 => loc::rdx(),
        _ => unreachable!(),
    }
}

fn extract_registers(locs: Vec<loc>) -> Vec<Register> {
    locs.into_iter()
        .map(|x| match x {
            loc::Deref(a, b, _, _) => vec![a, b].into_iter().filter_map(|x| x).collect(),
            loc::Register(i) => vec![i],
            _ => vec![],
        })
        .flatten()
        .collect()
}

/// given a loc, assign a register to that loc and emit the move required to place
/// the value into that register
fn loc_helper(
    location: loc,
    register_map: &HashMap<Register, loc>, //table that tell u where it is in memory
    register_names: &mut HashMap<Register, Register>, //translation table from temp to named register
    emit_ins: bool,
) -> (Vec<ins>, loc) {
    match location {
        loc::Deref(b, i, s, o) => {
            let (mut b_ins, b_new) = if let Some(base) = b {
                if register_names.contains_key(&base) {
                    (vec![], Some(register_names.get(&base).unwrap().clone()))
                } else if !register_map.contains_key(&base) {
                    (vec![], Some(base))
                } else {
                    let tmp = next_reg();
                    register_names.insert(base.clone(), tmp.unwrap_reg());
                    (
                        vec![(
                            OPCODE::mov,
                            tmp.clone(),
                            register_map.get(&base).unwrap().clone(),
                        )],
                        Some(tmp.unwrap_reg()),
                    )
                }
            } else {
                (vec![], None)
            };
            let (mut i_ins, i_new) = if let Some(index) = i {
                if register_names.contains_key(&index) {
                    (vec![], Some(register_names.get(&index).unwrap().clone()))
                } else if !register_map.contains_key(&index) {
                    (vec![], Some(index))
                } else {
                    let tmp = next_reg();
                    register_names.insert(index, tmp.unwrap_reg());
                    (
                        vec![(
                            OPCODE::mov,
                            tmp.clone(),
                            register_map.get(&index).unwrap().clone(),
                        )],
                        Some(tmp.unwrap_reg()),
                    )
                }
            } else {
                (vec![], None)
            };

            b_ins.append(&mut i_ins);
            (b_ins, loc::Deref(b_new, i_new, s, o))
        }
        loc::Register(i) => {
            if register_names.contains_key(&i) {
                (
                    vec![],
                    loc::Register(register_names.get(&i).unwrap().clone()),
                )
            } else if !register_map.contains_key(&i) {
                (vec![], loc::Register(i))
            } else if !emit_ins {
                (vec![], next_reg())
            } else {
                let tmp = next_reg();
                register_names.insert(i, tmp.unwrap_reg());
                (
                    vec![(
                        OPCODE::mov,
                        tmp.clone(),
                        register_map.get(&i).unwrap().clone(),
                    )],
                    tmp,
                )
            }
        }
        _ => (vec![], location),
    }
}

/// given an instruction in abstract assembly, replace it's registers with
/// rax, rdx, rcx and return an appropriate seq of instructions to move values
/// into the correct registers
fn conv_ins_to_named_reg(instruction: ins, register_map: &HashMap<Register, loc>) -> Vec<ins> {
    //println!("{:?}", instruction);
    let (opcode, dest, src) = instruction;

    //SPECIAL CASE TO CATCH MOVING IMM64 BRUH
    match (src.is_imm32(), &dest, &src) {
        (false, loc::Deref(_, _, _, _), loc::Literal(_))
        | (false, loc::Global(_), loc::Literal(_)) => {
            //println!("hey there");
            let mut seq = vec![];
            let mut register_names = HashMap::new();
            let (mut dest_ins, new_dest) = loc_helper(
                dest.clone(),
                register_map,
                &mut register_names,
                opcode.reads_dest() || dest.is_deref(),
            );
            seq.append(&mut dest_ins);
            let imm_temp = next_reg();
            seq.push((OPCODE::mov, imm_temp.clone(), src));
            seq.push((opcode, new_dest, imm_temp));
            return seq;
        }
        (false, loc::Register(r), loc::Literal(_)) => {
            let mut seq = vec![];
            let imm_temp = next_reg();
            seq.push((OPCODE::mov, imm_temp.clone(), src));
            match register_map.get(r) {
                Some(stack_loc) => seq.push((OPCODE::mov, stack_loc.clone(), imm_temp)),
                None => seq.push((OPCODE::mov, dest, imm_temp)),
            };
            return seq;
        }
        _ => (),
    };
    match (&opcode, &dest, &src) {
        (OPCODE::mov, _, _) => {
            //mov is a special case as it does not read its destination, and thus dest does not be moved from the stack into a register
            let mut seq = vec![];
            let mut register_names = HashMap::new();
            match dest {
                loc::Deref(_, _, _, _) => {
                    let (mut dest_ins, new_dest) = loc_helper(
                        dest.clone(),
                        register_map,
                        &mut register_names,
                        opcode.reads_dest() || dest.is_deref(),
                    );
                    seq.append(&mut dest_ins);

                    let (mut src_ins, new_src) =
                        loc_helper(src, register_map, &mut register_names, true);
                    seq.append(&mut src_ins);
                    seq.push((opcode, new_dest.clone(), new_src))
                }
                loc::Register(_) => {
                    //special case moving into rax, rdx, .. etc
                    if !register_map.contains_key(&dest.unwrap_reg()) {
                        if src.is_register() {
                            return match register_map.get(&src.unwrap_reg()) {
                                Some(stack_pos) => vec![(OPCODE::mov, dest, stack_pos.clone())],
                                None => vec![(OPCODE::mov, dest, src)],
                            };
                        } else {
                            return vec![(OPCODE::mov, dest, src)];
                        }
                    }

                    let (mut src_ins, new_src) =
                        loc_helper(src.clone(), register_map, &mut register_names, true);
                    seq.append(&mut src_ins);
                    // if src is not a mem op we can push on to stack directly
                    // next_reg indicates the loc that needs be moved to stack
                    let next_reg = if new_src.is_deref() || new_src.is_glob() {
                        let next_reg = next_reg();
                        seq.push((opcode, next_reg.clone(), new_src));
                        next_reg
                    } else {
                        new_src
                    };

                    //move on to stack
                    match register_map.get(&dest.unwrap_reg()) {
                        Some(stack_pos) => seq.push((OPCODE::mov, stack_pos.clone(), next_reg)),
                        None => (),
                    }
                }
                loc::Global(_) => {
                    let (mut src_ins, new_src) =
                        loc_helper(src.clone(), register_map, &mut register_names, true);
                    seq.append(&mut src_ins);
                    seq.push((OPCODE::mov, dest, new_src));
                }
                _ => {
                    unreachable!()
                }
            }
            seq
        }
        //handles translation of single operand imul and idiv so rdx:rax is not overwritten
        (OPCODE::idiv, loc::Register(r), loc::Null)
        | (OPCODE::imul, loc::Register(r), loc::Null) => {
            vec![match register_map.get(r) {
                Some(stack_pos) => (opcode, stack_pos.clone(), src),
                None => (opcode, dest, src),
            }]
        }
        _ => {
            let mut seq = vec![];

            let mut register_names = HashMap::new();
            let (mut dest_ins, new_dest) = loc_helper(
                dest.clone(),
                register_map,
                &mut register_names,
                opcode.reads_dest() || dest.is_deref(),
            );
            seq.append(&mut dest_ins);

            let (mut src_ins, new_src) = loc_helper(src, register_map, &mut register_names, true);
            seq.append(&mut src_ins);

            if opcode.is_set() {
                seq.push((opcode, new_dest.lower_byte(), new_src)); //set register
                seq.push((OPCODE::movsx, new_dest.clone(), new_dest.lower_byte()));
            //sign extend
            } else {
                seq.push((opcode, new_dest.clone(), new_src));
            }
            if opcode.is_set() || (opcode.mutates_dest() && !dest.is_deref()) {
                //need to push result back onto the stack in this case
                match register_map.get(&dest.unwrap_reg()) {
                    Some(stack_pos) => seq.push((OPCODE::mov, stack_pos.clone(), new_dest)),
                    None => (),
                }
            };
            seq
        }
    }
}

fn allocate_reg_trivial(seq: Vec<ins>, max_fn_args: usize) -> Vec<ins> {
    let seq = seq
        .into_iter()
        .map(|x| fix_double_mem_operand(x))
        .flatten()
        .collect::<Vec<_>>();
    let mut register_map = HashMap::new();
    seq.iter().for_each(|(_, src, dest)| {
        for register in extract_registers(vec![src.clone(), dest.clone()]) {
            if (!register.is_named()) && !register_map.contains_key(&register) {
                let pos = (register_map.len() as isize + 1) * -8;
                register_map.insert(
                    register,
                    loc::Deref(loc::rbp().unwrap_reg().into(), None, None, Some(pos)),
                );
            }
        }
    });

    let stack_size = register_map.len() + max_fn_args;

    let prologue = vec![
        (OPCODE::push, loc::rbp(), loc::null()), //pushes base pointer onto the stack
        (OPCODE::mov, loc::rbp(), loc::rsp()),   //change new base pointer to old stack pointer
        (
            OPCODE::sub,
            loc::rsp(),
            loc::Literal(8 * stack_size as isize),
        ), //allocate space on stack
        (OPCODE::and, loc::rsp(), loc::Literal(-16)),
    ];

    let body = seq
        .into_iter()
        .map(|x| conv_ins_to_named_reg(x, &mut register_map))
        .flatten();

    prologue
        .into_iter()
        .chain(body)
        .map(|x| match x {
            (OPCODE::EPILOGUE, _, _) => (OPCODE::leave, loc::null(), loc::null()),
            _ => x,
        })
        .collect()
}

fn max_fn_args(seq: &Vec<LIRStmt>) -> usize {
    seq.iter()
        .map(|x| match x {
            LIRStmt::Call(_, args, _) => args.len(),
            _ => 0,
        })
        .max()
        .unwrap_or(0)
}

#[allow(unused_assignments)]
pub fn translate_cu(lircu: LIRCompUnit, opt_options: &OptOptions) -> String {
    let LIRCompUnit {
        name: _,
        functions,
        interface_functions,
        data_map,
    } = lircu;
    let mut ttable =
        TTable::make_translation_table(data_map.keys().map(|x| format!("__{x}")).collect());
    let mut output = String::new();

    //directives
    writeln!(output, "\t.intel_syntax noprefix").unwrap();
    if data_map.len() > 0 {
        writeln!(output, "\t.data").unwrap();
    }
    //translating data map
    for (_, data) in data_map {
        writeln!(
            output,
            "{}",
            ins_to_string((OPCODE::LABEL, loc::Label(data.name), loc::null()))
        )
        .unwrap();
        if data.data.is_empty() {
            writeln!(output, "\t.zero 8").unwrap();
        } else {
            writeln!(output, "\t.quad {}", data.data[0]).unwrap();
        }
    }
    writeln!(output, "\t.text").unwrap();
    for i_f in interface_functions {
        writeln!(output, "\t.globl {}", i_f).unwrap();
    }
    for (fnname, decl) in functions {
        let LIRFuncDecl {
            body, n_returns, ..
        } = decl;
        //println!("{:?}", body);
        let max_fn_args = max_fn_args(&body);
        let seq = translate_block(body, n_returns, &mut ttable, opt_options);

        writeln!(output, "\t.globl {}", decl.name).unwrap();
        writeln!(
            output,
            "{}",
            ins_to_string((OPCODE::LABEL, loc::Label(decl.name), loc::null()))
        )
        .unwrap();
        let mut alloc_seq = vec![];
        if opt_options.reg {
            alloc_seq = regalloc::alloc_register_linear(seq, max_fn_args);
        } else {
            alloc_seq = allocate_reg_trivial(seq, max_fn_args);
        }
        for ins in &alloc_seq {
            writeln!(output, "{}", ins_to_string(ins.clone())).unwrap();
        }
    }
    output
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

        (_, loc::Deref(_, _, _, _), loc::Deref(_, _, _, _))
        | (_, loc::Global(_), loc::Deref(_, _, _, _))
        | (_, loc::Deref(_, _, _, _), loc::Global(_)) => {
            let temp = loc::temp();
            vec![
                (OPCODE::mov, temp.clone(), instruction.2),
                (instruction.0, instruction.1, temp),
            ]
        }
        _ => vec![instruction],
    }
}

fn translate_block(
    seq: Vec<LIRStmt>,
    n_returns: usize,
    ttable: &mut TTable,
    opt_options: &OptOptions,
) -> Vec<ins> {
    //println!("{:?}", seq);
    //sets flag in ttable to indicate arg count
    ttable.shift_args = n_returns > 2;
    reset();
    let lirtree = seq
        .into_iter()
        .map(|x| x.convert())
        .collect::<Vec<LIRNode>>();

    let size = next_int();
    let (lirtree, n_nodes) = if opt_options.dce || opt_options.copy {
        let lirbb_cfg = cfg::graph_from_bblocks(lirtree.clone());
        //loop_opt::loop_invariant_code_motion(&lirbb_cfg);

        let mut lirbb_ssa = CFG::convert_to_ssa(&lirbb_cfg);
        lirbb_ssa.recompute_defs();

        // if opt_options.copy {
        //     copyprop::copy_propagation(&mut lirbb_ssa);
        // }

        lirbb_ssa.convert_to_normal();
        let mut lirbb_ssa = lirbb_ssa.unwrap_SSA();
        let (mut res, size) = lirbb_ssa.flatten();
        if opt_options.dce {
            res = dce::eliminate_dead_code(res);
        }
        (res, size)
    } else {
        (lirtree, size)
    };
    //let v: Vec<LIRNode> = copyprop::copy_propagation(&lirtree);

    //lirbb_ssa.debug_cfg();

    //debug_cfg.debug_cfg();

    // if not passing into ssa form (-o flag), need to set n_nodes to correct size
    // if convert_to_ssa was not called, this is just next_int(), if it was called,
    // then the tree size must be recalculated.
    // let n_nodes = next_int();
    let mut memo: Vec<Option<Translation>> = vec![None; n_nodes];
    let mut vartable = HashMap::new();

    reset();
    //fill memo

    // for node in lirtree.iter() {
    //     println!("{:?}", node);
    // }
    //let lirtree = copyprop::copy_propagation(&lirtree);
    for node in lirtree.iter() {
        node.translate(&mut memo, &ttable, &mut vartable);
    }

    //println!("vartable: {:?}", vartable);
    lirtree
        .into_iter()
        .map(|x| {
            let mut translation = vec![];
            build_translation(&memo, x.get_idx(), &mut translation);
            translation
        })
        .flatten()
        .collect()
}

/// performs a post order traversal of the memo table to build a full translation
fn build_translation(memo: &Vec<Option<Translation>>, idx: usize, translation: &mut Vec<ins>) {
    let Translation { mut trans, opt, .. } = memo.get(idx).unwrap().clone().unwrap();
    for a in opt {
        build_translation(memo, a, translation)
    }
    translation.append(&mut trans);
}
