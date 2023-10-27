use crate::{
    cost::{ins_cost, total_cost},
    ir_types::Op,
    next_signed,
    translate_types::{ins, loc, LIRNode, Register, TTable, Translation, OPCODE},
};

use std::collections::{HashMap, HashSet};

impl TTable {
    pub fn make_translation_table(globals: HashSet<String>) -> TTable {
        TTable {
            globals,
            mov: vec![
                move_translation_args,
                // move_translation_0,
                move_translation_1,
                move_translation_2,
                move_translation_3,
            ],
            call: vec![call_translation_1],
            jump: vec![jump_translation_1],
            cjump: vec![cjump_translation_1, cjump_translation_2],
            ret: vec![ret_translation_1],
            op: vec![op_translation_1, op_translation_2],
            mem: vec![mem_translation_1, mem_translation_2],
            name: vec![name_translation_1],
            label: vec![label_translation],
            leaf: vec![leaf_translation],
            shift_args: false,
        }
    }
}
//UTILITIES

/// precondition: memo[i] is not None
fn get_translation(memo: &Vec<Option<Translation>>, i: usize) -> (usize, loc) {
    if let Some(x) = memo.get(i).unwrap() {
        (x.cost, x.target.clone())
    } else {
        unreachable!();
    }
}

pub fn is_fn_arg(arg: &String, ttable: &TTable) -> Option<usize> {
    if !arg.starts_with("_ARG") {
        return None;
    }
    (arg.chars()
        .filter(|x| x.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
        + ttable.shift_args as usize)
        .into()
}

pub fn is_return(arg: &String) -> Option<usize> {
    if !arg.starts_with("_RV") {
        return None;
    }
    arg.chars()
        .filter(|x| x.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
        .into()
}

fn op_to_opcode(op: &Op) -> OPCODE {
    match op {
        Op::Add => OPCODE::add,
        Op::Sub => OPCODE::sub,
        Op::Mul => OPCODE::imul,
        Op::HMul => OPCODE::imul,
        Op::Div => OPCODE::idiv,
        Op::Mod => OPCODE::idiv,
        Op::And => OPCODE::and,
        Op::Or => OPCODE::or,
        Op::Xor => OPCODE::xor,
        Op::LShift => OPCODE::shl,
        Op::RShift => OPCODE::shr,
        Op::Eq => OPCODE::sete,
        Op::Neq => OPCODE::setne,
        Op::Ult => OPCODE::setb,
        Op::Lt => OPCODE::setl,
        Op::Leq => OPCODE::setle,
        Op::Gt => OPCODE::setg,
        Op::Geq => OPCODE::setge,
        Op::Field => todo!(),
    }
}
//LABEL TRANSLATIONS
/// precondition: node is a LIRNode::Label
fn label_translation(
    node: &LIRNode,
    _memo: &mut Vec<Option<Translation>>,
    _ttable: &TTable,
    _vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Label { val, .. } = node {
        Some(Translation {
            trans: vec![(OPCODE::LABEL, loc::Label(val.clone()), loc::null())],
            opt: vec![],
            cost: 0,
            target: loc::null(),
        })
    } else {
        unreachable!("this function should only be called to translate LIRNode::Label")
    }
}

//NAME TRANSLATIONS
fn name_translation_1(
    node: &LIRNode,
    _memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Name { val, .. } = node {
        match vartable.get(val as &str) {
            Some(v) => Some(Translation::from_loc(loc::Register(v.clone()))),
            None => {
                if ttable.globals.contains(val) {
                    Some(Translation::from_loc(loc::Global(val.clone())))
                } else {
                    let rn = loc::temp();
                    vartable.insert(val.to_string(), rn.unwrap_reg());
                    Some(Translation::from_loc(rn))
                }
            }
        }
    } else {
        unreachable!("should only be called for name")
    }
}

fn move_translation_args(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Move { val: (_, r), .. } = node {
        r.translate(memo, ttable, vartable);
        if let LIRNode::Temp { val: tmp, .. } = &**r {
            match is_fn_arg(tmp, ttable) {
                None => None,
                Some(v) => {
                    if v <= 6 {
                        return None;
                    }
                    Some(Translation {
                        trans: vec![],
                        opt: vec![],
                        cost: 0,
                        target: loc::null(),
                    })
                }
            }
        } else {
            None
        }
    } else {
        unreachable!()
    }
}

//TEMP, CONST TRANSLATIONS
/// converts a leaf node (const, temp) to loc
fn leaf_translation(
    node: &LIRNode,
    _memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    match node {
        LIRNode::Const { val, .. } => Some(Translation::from_loc(loc::Literal(*val as isize))),
        LIRNode::Temp { val, .. } => match vartable.get(val as &str) {
            Some(v) => Some(Translation::from_loc(loc::Register(v.clone()))),
            None => {
                if let Some(argn) = is_fn_arg(val, ttable) {
                    let loc = match argn {
                        0 => unreachable!(),
                        1 => loc::rdi(),
                        2 => loc::rsi(),
                        3 => loc::rdx(),
                        4 => loc::rcx(),
                        5 => loc::r8(),
                        6 => loc::r9(),
                        _ => loc::Deref(
                            Some(Register::RBP),
                            None,
                            None,
                            Some((argn as isize - 6) * 8),
                        ),
                    };
                    Some(Translation::from_loc(loc))
                } else if let Some(retn) = is_return(val) {
                    let loc = match retn {
                        0 => unreachable!(),
                        1 => loc::rv1(),
                        2 => loc::rv2(),
                        _ => loc::Deref(
                            Some(Register::RDI),
                            None,
                            None,
                            Some(8 * (retn as isize - 2)),
                        ),
                    };
                    Some(Translation::from_loc(loc))
                } else {
                    let register = loc::temp();
                    vartable.insert(val.to_string(), register.unwrap_reg());
                    Some(Translation::from_loc(register))
                }
            }
        },
        _ => unreachable!(),
    }
}

// MOVE TRANSLATIONS
// should call translate!!
// precondition node is a LIRNode::Move
// returning none indicates that this translation is not valid
fn move_translation_1(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    //match left is register, right is op
    //produce a translation
    //left -> if we are moving into a register r, mov r (rhs) with last_mut r,
    //if we are moving into memory, then calling translate on LIRNode::mem will return an instruction sequence
    //with last_mut set the correct loc (a deref).
    //right -> call right.translate(..) -> Translation {Vec<ins>, cost, register}
    //append "mov next_int() register" (x86) to the Vec<ins>, add the cost of mov ins, return Translation
    if let LIRNode::Move { val: (l, r), .. } = node {
        l.translate(memo, ttable, vartable);
        r.translate(memo, ttable, vartable);
        let (lcost, dest) = get_translation(memo, l.get_idx());
        let (rcost, src) = get_translation(memo, r.get_idx());
        let new_ins = (OPCODE::mov, dest.clone(), src);
        if let loc::Global(name) = &dest {
            vartable.remove(name);
        }
        Some(Translation {
            trans: vec![new_ins],
            opt: vec![l.get_idx(), r.get_idx()],
            cost: lcost + rcost + OPCODE::mov.cost(),
            target: dest,
        })
    } else {
        unreachable!()
    }
}

fn move_translation_2(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Move { val: (l, r), .. } = node {
        l.translate(memo, ttable, vartable);
        let (lcost, dest) = get_translation(memo, l.get_idx());
        match &**r {
            LIRNode::Op {
                val: (op, lop, rop),
                ..
            } => {
                lop.translate(memo, ttable, vartable);
                rop.translate(memo, ttable, vartable);
                let (lopc, loploc) = get_translation(memo, lop.get_idx());
                let (ropc, roploc) = get_translation(memo, rop.get_idx());
                if loploc == dest {
                    match op {
                        Op::Add
                        | Op::Sub
                        | Op::Mul
                        | Op::And
                        | Op::Or
                        | Op::Xor
                        | Op::LShift
                        | Op::RShift => {
                            let opcode = op_to_opcode(&op);
                            let trans = vec![
                                (opcode, loploc, roploc), //perform operation
                            ];
                            let cost = lcost + lopc + ropc + total_cost(&trans);
                            Some(Translation {
                                trans: trans,
                                opt: vec![l.get_idx(), lop.get_idx(), rop.get_idx()],
                                cost,
                                target: dest,
                            })
                        }
                        _ => None,
                    }
                } else if roploc == dest {
                    match op {
                        Op::Add | Op::Mul | Op::And | Op::Or | Op::Xor => {
                            let opcode = op_to_opcode(&op);
                            let trans = vec![
                                (opcode, roploc, loploc), //perform operation
                            ];
                            let cost = lcost + lopc + ropc + total_cost(&trans);
                            Some(Translation {
                                trans: trans,
                                opt: vec![l.get_idx(), lop.get_idx(), rop.get_idx()],
                                cost,
                                target: dest,
                            })
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            LIRNode::Const { val, .. } => {
                let trans = vec![(OPCODE::mov, dest.clone(), loc::Literal(*val as isize))];
                let tc = total_cost(&trans);
                Some(Translation {
                    trans,
                    opt: vec![],
                    cost: tc,
                    target: dest,
                })
            }
            _ => None,
        }
    } else {
        unreachable!()
    }
}

fn move_translation_3(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Move { val: (l, r), .. } = node {
        l.translate(memo, ttable, vartable);
        let (lcost, dest) = get_translation(memo, l.get_idx());

        if !dest.is_register() {
            return None;
        }

        match mem_operand_form(r, memo, ttable, vartable) {
            None => None,
            Some(translation) => {
                let Translation {
                    mut trans,
                    opt,
                    target,
                    cost,
                } = translation;
                match target {
                    loc::Deref(a, b, _, _) => {
                        let registers =
                            vec![a, b].into_iter().filter_map(|x| x).collect::<Vec<_>>();

                        //asserted dest was register earlier
                        if registers.contains(&dest.unwrap_reg()) {
                            let lea_ins = (OPCODE::lea, dest.clone(), target);
                            let total_cost = lcost + cost + ins_cost(&lea_ins);
                            trans.push(lea_ins);
                            Some(Translation {
                                trans,
                                opt,
                                cost: total_cost,
                                target: dest,
                            })
                        } else {
                            None
                        }
                    }
                    _ => unreachable!("has to be deref"),
                }
            }
        }
    } else {
        unreachable!()
    }
}

fn _move_translation_0(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Move { val: (l, r), .. } = node {
        if let LIRNode::Call { .. } = &**r {
            return None;
        }
        match &**l {
            LIRNode::Name { val, .. } | LIRNode::Temp { val, .. } => {
                if vartable.contains_key(val) {
                    return None;
                }

                r.translate(memo, ttable, vartable);
                let (cost, rloc) = get_translation(memo, r.get_idx());

                let rtrans = memo.get(r.get_idx()).unwrap().clone().unwrap();
                match rtrans.trans.last() {
                    Some((opcode, dst, _)) => {
                        if opcode.mutates_dest() {
                            if !dst.is_register() || dst.clone() != rloc {
                                return None;
                            }
                            vartable.insert(val.clone(), dst.unwrap_reg());
                            l.translate(memo, ttable, vartable);
                            Some(Translation {
                                trans: vec![],
                                opt: vec![r.get_idx()],
                                cost,
                                target: rloc,
                            })
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
            _ => None,
        }
    } else {
        unreachable!()
    }
}

//CALL TRANSLATIONS
fn call_translation_1(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Call {
        val: (name, args, m),
        ..
    } = node
    {
        if *m <= 2 {
            let n = args.len();
            let mut trans = vec![];
            let mut opt = vec![];
            let arg_reg_vec = vec![
                loc::rdi(),
                loc::rsi(),
                loc::rdx(),
                loc::rcx(),
                loc::r8(),
                loc::r9(),
            ];
            let mut stack_arg_vec = vec![];
            let mut offset = 8;

            for i in 0..n {
                args[i].translate(memo, ttable, vartable);
                let (_, tmp) = get_translation(memo, args[i].get_idx());
                opt.push(args[i].get_idx());
                if i < 6 {
                    trans.push((OPCODE::mov, arg_reg_vec[i].clone(), tmp));
                } else {
                    //extra_reg_vec.push((OPCODE::push, tmp, loc::null()));
                    stack_arg_vec.push((
                        OPCODE::mov,
                        loc::Deref(Some(Register::RSP), None, None, Some(offset)),
                        tmp,
                    ));
                    offset += 8;
                }
            }
            stack_arg_vec.reverse();
            trans.extend(stack_arg_vec);
            trans.push((OPCODE::call(n), loc::Label(name.to_string()), loc::null()));
            if *m >= 1 {
                trans.push((OPCODE::mov, loc::rv1(), loc::rax()));
            }
            if *m >= 2 {
                trans.push((OPCODE::mov, loc::rv2(), loc::rdx()));
            }

            let cost = total_cost(&trans);
            Some(Translation {
                trans,
                opt,
                cost,
                target: loc::rv1(),
            })
        } else {
            let n = args.len();
            let mut trans = vec![];
            let mut opt = vec![];
            let arg_reg_vec = vec![loc::rsi(), loc::rdx(), loc::rcx(), loc::r8(), loc::r9()];
            let mut stack_arg_vec = vec![];
            let mut offset = 8;
            trans.push((OPCODE::sub, loc::rsp(), loc::Literal(8 * (m - 2) as isize)));
            trans.push((OPCODE::mov, loc::rdi(), loc::rsp()));
            for i in 0..n {
                args[i].translate(memo, ttable, vartable);
                let (_, tmp) = get_translation(memo, args[i].get_idx());
                opt.push(args[i].get_idx());
                if i < 5 {
                    trans.push((OPCODE::mov, arg_reg_vec[i].clone(), tmp));
                } else {
                    //extra_reg_vec.push((OPCODE::push, tmp, loc::null()));
                    stack_arg_vec.push((
                        OPCODE::mov,
                        loc::Deref(Some(Register::RSP), None, None, Some(offset)),
                        tmp,
                    ));
                    offset += 8;
                }
            }
            stack_arg_vec.reverse();
            trans.extend(stack_arg_vec);
            trans.push((OPCODE::call(n), loc::Label(name.to_string()), loc::null()));
            if *m > 2 {
                trans.push((OPCODE::add, loc::rsp(), loc::Literal(8 * (m - 2) as isize)));
            }
            if *m >= 1 {
                trans.push((OPCODE::mov, loc::rv1(), loc::rax()));
            }
            if *m >= 2 {
                trans.push((OPCODE::mov, loc::rv2(), loc::rdx()));
            }

            let cost = total_cost(&trans);
            Some(Translation {
                trans,
                opt,
                cost,
                target: loc::rv1(),
            })
        }
    } else {
        unreachable!("Should not try to translate a call from an LIRNode that's not a call")
    }
}

//RETURN TRANSLATIONS
fn ret_translation_1(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Return { val: returns, .. } = node {
        let m = returns.len();
        let mut trans = vec![];
        let mut opt = vec![];
        let ret_register_vec = vec![loc::rax(), loc::rdx()];

        for i in 0..m {
            returns[i].translate(memo, ttable, vartable);
            let (_, tmp) = get_translation(memo, returns[i].get_idx());
            //let tmp = loc::Register(-14 - i as isize);
            opt.push(returns[i].get_idx());
            if i < 2 {
                trans.push((OPCODE::mov, ret_register_vec[i].clone(), tmp));
            }
        }
        //placeholder for epilogue
        trans.push((OPCODE::EPILOGUE, loc::null(), loc::null()));
        //trans.push((OPCODE::leave, loc::null(), loc::null()));
        trans.push((OPCODE::ret, loc::null(), loc::null()));
        let cost = total_cost(&trans);

        Some(Translation {
            trans,
            opt,
            cost,
            target: loc::null(),
        })
    } else {
        unreachable!("Should not be trying to translate a return if it's not a return")
    }
}

//JUMP TRANSLATIONS
fn jump_translation_1(
    node: &LIRNode,
    _memo: &mut Vec<Option<Translation>>,
    _ttable: &TTable,
    _vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Jump { val, .. } = node {
        let new_ins = (OPCODE::jmp, loc::Label(val.clone()), loc::null());
        Some(Translation {
            trans: vec![new_ins],
            opt: vec![],
            cost: OPCODE::jmp.cost(),
            target: loc::null(),
        })
    } else {
        unreachable!("Should not call this translation on an IRNode that isn't a jump")
    }
}

fn cmp_helper(a: loc, b: loc) -> Vec<ins> {
    match (&a, &b) {
        (loc::Literal(_), loc::Literal(_)) => {
            let dr = loc::temp();
            let sr = loc::temp();
            vec![
                (OPCODE::mov, dr.clone(), a), //mov a into register
                (OPCODE::mov, sr.clone(), b), //mov a into register
                (OPCODE::cmp, dr, sr),
            ]
        }
        (loc::Literal(_), _) => {
            let dr = loc::temp();
            vec![
                (OPCODE::mov, dr.clone(), a), //mov a into register
                (OPCODE::cmp, dr, b),
            ]
        }
        (_, loc::Literal(_)) => {
            let sr = loc::temp();
            vec![
                (OPCODE::mov, sr.clone(), b), //mov a into register
                (OPCODE::cmp, a, sr),
            ]
        }
        _ => vec![(OPCODE::cmp, a, b)],
    }
}

/// unoptimized cjump translation
fn cjump_translation_1(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::CJump {
        val: (cond, label), ..
    } = node
    {
        cond.translate(memo, ttable, vartable);
        let (cost, val) = get_translation(memo, cond.get_idx());
        let trans = vec![
            (OPCODE::test, val.clone(), val.clone()), //set ZF
            (OPCODE::jnz, loc::Label(label.clone()), loc::null()), //jump if ZF = 1
        ];
        let total_cost = cost + total_cost(&trans);
        Some(Translation {
            trans,
            opt: vec![cond.get_idx()],
            cost: total_cost,
            target: loc::null(),
        })
    } else {
        unreachable!()
    }
}

fn cjump_translation_2(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::CJump {
        val: (cond, label), ..
    } = node
    {
        match &**cond {
            LIRNode::Op {
                val: (op, l, r), ..
            } => {
                l.translate(memo, ttable, vartable);
                r.translate(memo, ttable, vartable);
                let (lcost, dest) = get_translation(memo, l.get_idx());
                let (rcost, src) = get_translation(memo, r.get_idx());
                match &*op {
                    Op::Gt => {
                        let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                        cmp_ins.push((OPCODE::jg, loc::Label(label.clone()), loc::null()));
                        Some(Translation {
                            trans: cmp_ins,
                            opt: vec![l.get_idx(), r.get_idx()],
                            cost: lcost + rcost + OPCODE::cmp.cost() + OPCODE::jg.cost(),
                            target: loc::Label(label.clone()),
                        })
                    }
                    Op::Geq => {
                        let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                        let jge_ins = (OPCODE::jge, loc::Label(label.clone()), loc::null());
                        cmp_ins.push(jge_ins);
                        Some(Translation {
                            trans: cmp_ins,
                            opt: vec![l.get_idx(), r.get_idx()],
                            cost: lcost + rcost + OPCODE::cmp.cost() + OPCODE::jge.cost(),
                            target: loc::Label(label.clone()),
                        })
                    }
                    Op::Lt => {
                        let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                        let jl_ins = (OPCODE::jl, loc::Label(label.clone()), loc::null());
                        cmp_ins.push(jl_ins);
                        Some(Translation {
                            trans: cmp_ins,
                            opt: vec![l.get_idx(), r.get_idx()],
                            cost: lcost + rcost + OPCODE::cmp.cost() + OPCODE::jl.cost(),
                            target: loc::Label(label.clone()),
                        })
                    }
                    Op::Leq => {
                        let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                        let jle_ins = (OPCODE::jle, loc::Label(label.clone()), loc::null());
                        cmp_ins.push(jle_ins);
                        Some(Translation {
                            trans: cmp_ins,
                            opt: vec![l.get_idx(), r.get_idx()],
                            cost: lcost + rcost + OPCODE::cmp.cost() + OPCODE::jle.cost(),
                            target: loc::Label(label.clone()),
                        })
                    }
                    Op::Eq => {
                        let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                        let je_ins = (OPCODE::je, loc::Label(label.clone()), loc::null());
                        cmp_ins.push(je_ins);
                        Some(Translation {
                            trans: cmp_ins,
                            opt: vec![l.get_idx(), r.get_idx()],
                            cost: lcost + rcost + OPCODE::cmp.cost() + OPCODE::je.cost(),
                            target: loc::Label(label.clone()),
                        })
                    }
                    Op::Neq => {
                        let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                        let jne_ins = (OPCODE::jne, loc::Label(label.clone()), loc::null());
                        cmp_ins.push(jne_ins);
                        Some(Translation {
                            trans: cmp_ins,
                            opt: vec![l.get_idx(), r.get_idx()],
                            cost: lcost + rcost + OPCODE::cmp.cost() + OPCODE::jne.cost(),
                            target: loc::Label(label.clone()),
                        })
                    }
                    _ => None,
                }
            }
            LIRNode::Temp { .. } => {
                cond.translate(memo, ttable, vartable);
                let (cost, val) = get_translation(memo, cond.get_idx());
                let test_ins = (OPCODE::test, val.clone(), val);
                let jnz_ins = (OPCODE::jnz, loc::Label(label.clone()), loc::null());
                Some(Translation {
                    trans: vec![test_ins, jnz_ins],
                    opt: vec![cond.get_idx()],
                    cost: cost + OPCODE::test.cost() + OPCODE::jnz.cost(),
                    target: loc::Label(label.clone()),
                })
            }
            LIRNode::Const { val, .. } => {
                if *val == 1 {
                    Some(Translation {
                        trans: vec![(OPCODE::jmp, loc::Label(label.clone()), loc::null())],
                        opt: vec![],
                        cost: OPCODE::jmp.cost(),
                        target: loc::Label(label.clone()),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        unreachable!();
    }
}

//OP TRANSLATION
/// translates op and puts result in target
fn op_translation_1(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Op {
        val: (op, l, r), ..
    } = node
    {
        let opcode = op_to_opcode(op);
        l.translate(memo, ttable, vartable);
        r.translate(memo, ttable, vartable);
        let (lcost, dest) = get_translation(memo, l.get_idx());
        let (rcost, src) = get_translation(memo, r.get_idx());
        let temp = loc::temp();
        match op {
            Op::Add | Op::Sub | Op::Mul | Op::And | Op::Or | Op::Xor | Op::LShift | Op::RShift => {
                let trans = vec![
                    (OPCODE::mov, temp.clone(), dest), //move dest to temp
                    (opcode, temp.clone(), src),       //perform operation
                ];
                Some(Translation {
                    trans,
                    opt: vec![l.get_idx(), r.get_idx()],
                    cost: lcost + rcost + opcode.cost() + OPCODE::mov.cost(),
                    target: temp,
                })
            }
            Op::Div | Op::Mod => {
                let res = if op.clone() == Op::Div {
                    loc::rax()
                } else {
                    loc::rdx()
                };
                let trans = match &src {
                    loc::Literal(_) => {
                        let src_temp = loc::temp();
                        vec![
                            (OPCODE::mov, loc::rax(), dest),         //move dest to rax
                            (OPCODE::cqo, loc::null(), loc::null()), //sign extend into rdx
                            (OPCODE::mov, src_temp.clone(), src),
                            (OPCODE::idiv, src_temp, loc::null()), //perform division
                            (OPCODE::mov, temp.clone(), res),      //put result in temp
                        ]
                    }
                    _ => {
                        vec![
                            (OPCODE::mov, loc::rax(), dest),         //move dest to rax
                            (OPCODE::cqo, loc::null(), loc::null()), //sign extend into rdx
                            (OPCODE::idiv, src, loc::null()),        //perform division
                            (OPCODE::mov, temp.clone(), res),        //put result in temp
                        ]
                    }
                };
                let cost = lcost + rcost + total_cost(&trans);
                Some(Translation {
                    trans,
                    opt: vec![l.get_idx(), r.get_idx()],
                    cost,
                    target: temp,
                })
            }
            Op::Eq | Op::Neq | Op::Lt | Op::Leq | Op::Gt | Op::Geq | Op::Ult => {
                let mut cmp_ins = cmp_helper(dest.clone(), src.clone());
                cmp_ins.push((opcode, temp.clone(), loc::null()));
                let cost = lcost + rcost + total_cost(&cmp_ins);
                Some(Translation {
                    trans: cmp_ins,
                    opt: vec![l.get_idx(), r.get_idx()],
                    cost,
                    target: temp,
                })
            }
            Op::HMul => {
                let trans = match &src {
                    loc::Literal(_) => {
                        let src_temp = loc::temp();
                        vec![
                            (OPCODE::mov, loc::rax(), dest),         //move into rax
                            (OPCODE::mov, src_temp.clone(), src),    //move literal into register
                            (OPCODE::imul, src_temp, loc::null()),   //perform quadword signed mul
                            (OPCODE::mov, temp.clone(), loc::rdx()), // move high 64 bits into temp
                        ]
                    }
                    _ => {
                        vec![
                            (OPCODE::mov, loc::rax(), dest),         //move into rax
                            (OPCODE::imul, src, loc::null()),        //perform quadword signed mul
                            (OPCODE::mov, temp.clone(), loc::rdx()), // move high 64 bits into temp
                        ]
                    }
                };

                let cost = lcost + rcost + total_cost(&trans);
                Some(Translation {
                    trans,
                    opt: vec![l.get_idx(), r.get_idx()],
                    cost,
                    target: temp,
                })
            }
            Op::Field => todo!()
        }
    } else {
        unreachable!()
    }
}

fn op_translation_2(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    match mem_operand_form(node, memo, ttable, vartable) {
        None => None,
        Some(translation) => {
            let Translation {
                mut trans,
                opt,
                target,
                cost,
            } = translation;
            let temp = loc::temp();

            let lea_ins = (OPCODE::lea, temp.clone(), target);
            //println!("{:?}", lea_ins);
            let total_cost = cost + ins_cost(&lea_ins);
            trans.push(lea_ins);
            Some(Translation {
                trans,
                opt,
                cost: total_cost,
                target: temp,
            })
        }
    }
}

//MEM TRANSLATIONS
fn mem_translation_1(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Mem { val, .. } = node {
        val.translate(memo, ttable, vartable);
        let (cost, target) = get_translation(memo, val.get_idx());
        Some(match target {
            loc::Deref(_, _, _, _) => {
                let temp = loc::temp();
                let trans = vec![(OPCODE::lea, temp.clone(), target)];
                let cost = cost + total_cost(&trans);
                Translation {
                    trans,
                    opt: vec![val.get_idx()],
                    cost,
                    target: temp,
                }
            }
            loc::Register(r) => Translation {
                trans: vec![],
                opt: vec![val.get_idx()],
                cost: cost,
                target: loc::Deref(Some(r), None, None, None),
            },
            _ => unreachable!("mem should never contain {:?}", target),
        })
    } else {
        unreachable!("should only call this on a mem")
    }
}

/// returns in the form ins, reg, cost
fn mem_tr_reg_helper(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> (Option<ins>, Register, usize) {
    match node {
        LIRNode::Const { val, idx } => {
            memo[idx.clone()] = Some(Translation::from_loc(loc::Literal(*val as isize)));
            let temp = next_signed();
            (
                Some((
                    OPCODE::mov,
                    loc::temp_from_int(temp),
                    loc::Literal(*val as isize),
                )),
                Register::Temp(temp),
                0, //ignore cost, will be computed later
            )
        }
        _ => {
            node.translate(memo, ttable, vartable);
            let (cost, loc) = get_translation(memo, node.get_idx());
            match &loc {
                loc::Deref(_, _, _, _) => {
                    let temp = loc::temp();
                    (
                        Some((OPCODE::lea, temp.clone(), loc)),
                        temp.unwrap_reg(),
                        cost,
                    )
                }
                loc::Register(_) => (None, loc.unwrap_reg(), cost),
                _ => unreachable!("THIS IS UNREACHABLE FOR A REASON MFS IT SHOULDN'T BE REACHED"),
            }
        }
    }
}

/// takes in an LIRNode::Op and returns it in memory operand form if it can be translated as such, None otherwise
fn mem_operand_form(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Op {
        val: (op, _, _), ..
    } = node
    {
        if *op != Op::Add {
            return None;
        }
        if let Some((a, b, s, o)) = is_deref_full(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            let res_b = mem_tr_reg_helper(&b, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }
            if let Some(ins) = &res_b.0 {
                trans.push(ins.clone())
            } else {
                opt.push(b.get_idx());
            }
            let cost = total_cost(&trans) + res_a.2 + res_b.2;
            let scalar = match s {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            let offset = match o {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(
                    Some(res_a.1),
                    Some(res_b.1),
                    Some(scalar as isize),
                    Some(offset as isize),
                ),
            });
        }

        if let Some((a, s, o)) = is_deref_am_one_const(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }

            let cost = total_cost(&trans) + res_a.2;
            let offset = match o {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            let scalar = match s {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(
                    None,
                    Some(res_a.1),
                    Some(scalar as isize),
                    Some(offset as isize),
                ),
            });
        }
        if let Some((a, b, o)) = is_deref_aa(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            let res_b = mem_tr_reg_helper(&b, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }
            if let Some(ins) = &res_b.0 {
                trans.push(ins.clone())
            } else {
                opt.push(b.get_idx());
            }
            let cost = total_cost(&trans) + res_a.2 + res_b.2;
            let offset = match o {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(Some(res_a.1), Some(res_b.1), None, Some(offset as isize)),
            });
        }
        if let Some((a, b, s)) = is_deref_am_any(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            let res_b = mem_tr_reg_helper(&b, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }
            if let Some(ins) = &res_b.0 {
                trans.push(ins.clone())
            } else {
                opt.push(b.get_idx());
            }
            let cost = total_cost(&trans) + res_a.2 + res_b.2;
            let scalar = match s {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(Some(res_a.1), Some(res_b.1), Some(scalar as isize), None),
            });
        }

        if let Some((a, s)) = is_deref_mul_form(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }
            let cost = total_cost(&trans) + res_a.2;
            let scalar = match s {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(None, Some(res_a.1), Some(scalar as isize), None),
            });
        }

        if let Some((a, o)) = is_deref_add_one_const(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }
            let cost = total_cost(&trans) + res_a.2;
            let offset = match o {
                LIRNode::Const { val, .. } => val,
                _ => unreachable!(),
            };
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(None, Some(res_a.1), None, Some(offset as isize)),
            });
        }

        if let Some((a, b)) = is_deref_add_any(node) {
            let mut trans = vec![];
            let mut opt = vec![];
            let res_a = mem_tr_reg_helper(&a, memo, ttable, vartable);
            let res_b = mem_tr_reg_helper(&b, memo, ttable, vartable);
            if let Some(ins) = &res_a.0 {
                trans.push(ins.clone())
            } else {
                opt.push(a.get_idx());
            }
            if let Some(ins) = &res_b.0 {
                trans.push(ins.clone())
            } else {
                opt.push(b.get_idx());
            }
            let cost = total_cost(&trans) + res_a.2 + res_b.2;
            return Some(Translation {
                trans,
                opt,
                cost,
                target: loc::Deref(Some(res_a.1), Some(res_b.1), None, None),
            });
        }
    }
    return None;
}

/// Attemtps to translate an Mem of an Op using memory operands
fn mem_translation_2(
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) -> Option<Translation> {
    if let LIRNode::Mem { val, .. } = node {
        mem_operand_form(val, memo, ttable, vartable)
    } else {
        unreachable!("should only call this on a mem")
    }
}

// two op forms
fn is_deref_mul_form(a: &LIRNode) -> Option<(LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op == Op::Mul {
                match (&**b, &**c) {
                    (LIRNode::Const { val, .. }, _) => {
                        if *val < 0 {
                            return None;
                        }
                        Some((*c.clone(), *b.clone()))
                    }
                    (_, LIRNode::Const { val, .. }) => {
                        if *val < 0 {
                            return None;
                        }
                        Some((*b.clone(), *c.clone()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_deref_add_one_const(a: &LIRNode) -> Option<(LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op == Op::Add {
                match (&**b, &**c) {
                    (LIRNode::Const { .. }, _) => Some((*c.clone(), *b.clone())),
                    (_, LIRNode::Const { .. }) => Some((*b.clone(), *c.clone())),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_deref_add_any(a: &LIRNode) -> Option<(LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op == Op::Add {
                Some((*b.clone(), *c.clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}

//triple deref forms
//index_reg, scalar, offset
fn is_deref_am_one_const(a: &LIRNode) -> Option<(LIRNode, LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op == Op::Add {
                if let (LIRNode::Const { .. }, Some((index_register, scalar))) =
                    (&**b, is_deref_mul_form(c))
                {
                    Some((index_register, scalar, *b.clone()))
                } else if let (LIRNode::Const { .. }, Some((index_register, scalar))) =
                    (&**c, is_deref_mul_form(b))
                {
                    Some((index_register, scalar, *c.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

//a, index_reg, scalar
fn is_deref_am_any(a: &LIRNode) -> Option<(LIRNode, LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op == Op::Add {
                if let (_, Some((index_register, scalar))) = (&**b, is_deref_mul_form(c)) {
                    Some((*b.clone(), index_register, scalar))
                } else if let (_, Some((index_register, scalar))) = (&**c, is_deref_mul_form(b)) {
                    Some((*c.clone(), index_register, scalar))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

//a1, a2, offset
fn is_deref_aa(a: &LIRNode) -> Option<(LIRNode, LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op != Op::Add {
                return None;
            }
            match (&**b, &**c) {
                (LIRNode::Const { .. }, _) => {
                    if let Some((l, r)) = is_deref_add_any(c) {
                        return Some((l, r, *b.clone()));
                    }
                }
                (_, LIRNode::Const { .. }) => {
                    if let Some((l, r)) = is_deref_add_any(b) {
                        return Some((l, r, *c.clone()));
                    }
                }
                _ => {
                    if let Some((r, offset)) = is_deref_add_one_const(b) {
                        return Some((*c.clone(), r, offset));
                    } else if let Some((r, offset)) = is_deref_add_one_const(c) {
                        return Some((*b.clone(), r, offset));
                    }
                }
            }
        }
        _ => (),
    }
    None
}

//full deref form
fn is_deref_full(a: &LIRNode) -> Option<(LIRNode, LIRNode, LIRNode, LIRNode)> {
    match a {
        LIRNode::Op {
            val: (a_op, b, c), ..
        } => {
            if *a_op != Op::Add {
                return None;
            }
            match (&**b, &**c) {
                (LIRNode::Const { .. }, _) => {
                    if let Some((a, ir, scalar)) = is_deref_am_any(c) {
                        return Some((a, ir, scalar, *b.clone()));
                    }
                }
                (_, LIRNode::Const { .. }) => {
                    if let Some((a, ir, scalar)) = is_deref_am_any(b) {
                        return Some((a, ir, scalar, *c.clone()));
                    }
                }
                _ => {
                    if let Some((r, scalar, offset)) = is_deref_am_one_const(b) {
                        return Some((*b.clone(), r, scalar, offset));
                    } else if let Some((r, scalar, offset)) = is_deref_am_one_const(c) {
                        return Some((*c.clone(), r, scalar, offset));
                    }
                }
            }
        }
        _ => (),
    }
    None
}
