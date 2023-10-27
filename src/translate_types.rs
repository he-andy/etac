#![allow(non_camel_case_types)]
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fmt, format,
    hash::Hasher,
};

use crate::cfg::Var;
use crate::{
    ir_types::{LIRDest, LIRExpr, LIRStmt, Op},
    next_int, next_signed,
};

pub type translationfn = fn(
    &LIRNode,
    &mut Vec<Option<Translation>>,
    &TTable,
    &mut HashMap<String, Register>,
) -> Option<Translation>;

pub struct TTable {
    pub globals: HashSet<String>,
    pub mov: Vec<translationfn>,
    pub jump: Vec<translationfn>,
    pub cjump: Vec<translationfn>,
    pub ret: Vec<translationfn>,
    pub op: Vec<translationfn>,
    pub mem: Vec<translationfn>,
    pub call: Vec<translationfn>,
    pub name: Vec<translationfn>,
    pub label: Vec<translationfn>,
    pub leaf: Vec<translationfn>,
    pub shift_args: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Register {
    Temp(isize),
    RAX,
    RCX,
    RDX,
    RBX,
    RSP, //stack pointer
    RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    //LOWER BYTE REGISTERS
    AL,
    CL,
    DL,
    BL,
    SP,
    BPL,
    SIL,
    DIL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B,
    RV1,
    RV2,
}

impl Register {
    pub fn is_callee_saved(&self) -> bool {
        match self {
            Register::RBP => true,
            Register::RBX => true,
            Register::R12 => true,
            Register::R13 => true,
            Register::R14 => true,
            Register::R15 => true,
            _ => false,
        }
    }

    pub fn is_caller_saved(&self) -> bool {
        match self {
            Register::RAX => true,
            Register::RCX => true,
            Register::RDX => true,
            Register::RSI => true,
            Register::RDI => true,
            Register::R8 => true,
            Register::R9 => true,
            Register::R10 => true,
            Register::R11 => true,
            _ => false,
        }
    }

    pub fn is_named(&self) -> bool {
        match self {
            Register::Temp(_) => false,
            Register::RV1 | Register::RV2 => false,
            _ => true,
        }
    }

    pub fn lower_byte(&self) -> Self {
        match self {
            Register::RAX => Register::AL,
            Register::RCX => Register::CL,
            Register::RDX => Register::DL,
            Register::RBX => Register::BL,
            Register::RSP => Register::SP,
            Register::RBP => Register::BPL,
            Register::RSI => Register::SIL,
            Register::RDI => Register::DIL,
            Register::R8 => Register::R8B,
            Register::R9 => Register::R9B,
            Register::R10 => Register::R10B,
            Register::R11 => Register::R11B,
            Register::R12 => Register::R12B,
            Register::R13 => Register::R13B,
            Register::R14 => Register::R14B,
            Register::R15 => Register::R15B,
            _ => unreachable!("Cannot lower byte of {:?}", self),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Register::Temp(_) => unreachable!(),
            Register::RAX => "rax".to_string(),
            Register::RCX => "rcx".to_string(),
            Register::RDX => "rdx".to_string(),
            Register::RBX => "rbx".to_string(),
            Register::RSP => "rsp".to_string(),
            Register::RBP => "rbp".to_string(),
            Register::RSI => "rsi".to_string(),
            Register::RDI => "rdi".to_string(),
            Register::R8 => "r8".to_string(),
            Register::R9 => "r9".to_string(),
            Register::R10 => "r10".to_string(),
            Register::R11 => "r11".to_string(),
            Register::R12 => "r12".to_string(),
            Register::R13 => "r13".to_string(),
            Register::R14 => "r14".to_string(),
            Register::R15 => "r15".to_string(),
            Register::AL => "al".to_string(),
            Register::CL => "cl".to_string(),
            Register::DL => "dl".to_string(),
            Register::BL => "bl".to_string(),
            Register::SP => "sp".to_string(),
            Register::BPL => "bpl".to_string(),
            Register::SIL => "sil".to_string(),
            Register::DIL => "dil".to_string(),
            Register::R8B => "r8b".to_string(),
            Register::R9B => "r9b".to_string(),
            Register::R10B => "r10b".to_string(),
            Register::R11B => "r11b".to_string(),
            Register::R12B => "r12b".to_string(),
            Register::R13B => "r13b".to_string(),
            Register::R14B => "r14b".to_string(),
            Register::R15B => "r15b".to_string(),
            Register::RV1 => "rv1".to_string(),
            Register::RV2 => "rv2".to_string(),
        }
    }
}
//RAX(-1) // RCX(-2) //RDX(-3) //RBX(-4)  //RSP(-5)	//RBP(-6)	//RSI(-7)	//RDI(-8)
#[derive(Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum loc {
    Deref(
        Option<Register>,
        Option<Register>,
        Option<isize>,
        Option<isize>,
    ), //base, index, scale, signed offset (similar to at&t syntax)
    Literal(isize),
    Register(Register),
    Label(String),
    Global(String),
    Null,
}

impl loc {
    pub fn null() -> Self {
        loc::Null
    }

    pub fn temp() -> Self {
        loc::Register(Register::Temp(next_signed()))
    }

    pub fn temp_from_int(i: isize) -> Self {
        loc::Register(Register::Temp(i))
    }

    pub fn rax() -> Self {
        loc::Register(Register::RAX)
    }

    pub fn rcx() -> Self {
        loc::Register(Register::RCX)
    }

    pub fn rdx() -> Self {
        loc::Register(Register::RDX)
    }

    pub fn _rbx() -> Self {
        loc::Register(Register::RBX)
    }

    pub fn rsp() -> Self {
        loc::Register(Register::RSP)
    }

    pub fn rbp() -> Self {
        loc::Register(Register::RBP)
    }

    pub fn rsi() -> Self {
        loc::Register(Register::RSI)
    }

    pub fn rdi() -> Self {
        loc::Register(Register::RDI)
    }

    pub fn r8() -> Self {
        loc::Register(Register::R8)
    }

    pub fn r9() -> Self {
        loc::Register(Register::R9)
    }

    pub fn rv1() -> Self {
        loc::Register(Register::RV1)
    }

    pub fn rv2() -> Self {
        loc::Register(Register::RV2)
    }

    pub fn unwrap_reg(&self) -> Register {
        match self {
            loc::Register(v) => v.clone(),
            _ => unreachable!(),
        }
    }

    pub fn unwrap_literal(&self) -> isize {
        match self {
            loc::Literal(v) => v.clone(),
            _ => unreachable!(),
        }
    }

    pub fn is_deref(&self) -> bool {
        match self {
            loc::Deref(_, _, _, _) => true,
            _ => false,
        }
    }

    pub fn is_register(&self) -> bool {
        match self {
            loc::Register(_) => true,
            _ => false,
        }
    }

    pub fn is_named_reg(&self) -> bool {
        match self {
            loc::Register(v) => v.is_named(),
            _ => false,
        }
    }

    pub fn is_glob(&self) -> bool {
        match self {
            loc::Global(_) => true,
            _ => false,
        }
    }

    pub fn is_imm32(&self) -> bool {
        match self {
            loc::Literal(v) => match i32::try_from(*v) {
                Ok(_) => true,
                Err(_) => false,
            },
            _ => false,
        }
    }

    pub fn label_as_string(&self) -> String {
        match self {
            loc::Label(s) => s.clone(),
            _ => unreachable!(),
        }
    }

    pub fn lower_byte(&self) -> loc {
        match self {
            loc::Register(v) => loc::Register(v.lower_byte()),
            _ => unreachable!(),
        }
    }
}

//Translation Type
#[derive(Clone, Debug)]
pub struct Translation {
    pub trans: Vec<ins>,
    pub opt: Vec<usize>,
    pub cost: usize,
    pub target: loc,
}

impl Translation {
    pub fn from_loc(loc: loc) -> Self {
        Self {
            trans: vec![],
            opt: vec![],
            cost: 0,
            target: loc,
        }
    }
    pub fn _get_loc(&self) -> loc {
        return self.target.clone();
    }
}

pub type ins = (OPCODE, loc, loc);

#[derive(Clone, Copy, Debug)]
pub enum OPCODE {
    EPILOGUE, //placeholder for epilogue replacement
    LABEL, //LABEL opcode does not refer to an x86 instruction, but simply labels the code with the value in loc
    mov,
    add,
    sub,
    mul,
    div,
    iadd,
    isub,
    imul,
    imulq,
    idiv,
    inc,
    dec,
    and,
    or,
    xor,
    not,
    shl,
    shr,
    sar,
    jmp,
    jz,
    je,
    jnz,
    jne,
    jl,
    jle,
    jg,
    jge,
    jb,
    jbe,
    ja,
    jae,
    push,
    pop,
    test,
    cmp,
    call(usize),
    ret,
    sete,
    setne,
    setl,
    setg,
    setle,
    setge,
    lea,
    setb,
    cqo,
    leave,
    movsx,
}

impl OPCODE {
    //minimize # of ins,
    // TODO: implement better cost function later
    pub fn cost(&self) -> usize {
        1
    }

    pub fn is_set(&self) -> bool {
        match self {
            OPCODE::sete => true,
            OPCODE::setne => true,
            OPCODE::setl => true,
            OPCODE::setg => true,
            OPCODE::setle => true,
            OPCODE::setge => true,
            OPCODE::setb => true,
            _ => false,
        }
    }

    pub fn reads_dest(&self) -> bool {
        match self {
            OPCODE::mov => false,
            _ => true,
        }
    }

    pub fn mutates_dest(&self) -> bool {
        match self {
            OPCODE::LABEL => false,
            OPCODE::mov => true,
            OPCODE::add => true,
            OPCODE::sub => true,
            OPCODE::mul => true,
            OPCODE::iadd => true,
            OPCODE::isub => true,
            OPCODE::imul => true,
            OPCODE::inc => true,
            OPCODE::dec => true,
            OPCODE::and => true,
            OPCODE::or => true,
            OPCODE::xor => true,
            OPCODE::not => true,
            OPCODE::shl => true,
            OPCODE::shr => true,
            OPCODE::sar => true,
            OPCODE::lea => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            OPCODE::LABEL => "label",
            OPCODE::mov => "mov",
            OPCODE::add => "add",
            OPCODE::sub => "sub",
            OPCODE::mul => "mul",
            OPCODE::div => "div",
            OPCODE::inc => "inc",
            OPCODE::dec => "dec",
            OPCODE::and => "and",
            OPCODE::or => "or",
            OPCODE::xor => "xor",
            OPCODE::not => "not",
            OPCODE::shl => "shl",
            OPCODE::shr => "shr",
            OPCODE::sar => "sar",
            OPCODE::jmp => "jmp",
            OPCODE::jz => "jz",
            OPCODE::je => "je",
            OPCODE::jnz => "jnz",
            OPCODE::jne => "jne",
            OPCODE::jl => "jl",
            OPCODE::jle => "jle",
            OPCODE::jg => "jg",
            OPCODE::jge => "jge",
            OPCODE::jb => "jb",
            OPCODE::jbe => "jbe",
            OPCODE::ja => "ja",
            OPCODE::jae => "jae",
            OPCODE::push => "push",
            OPCODE::pop => "pop",
            OPCODE::test => "test",
            OPCODE::cmp => "cmp",
            OPCODE::call(_) => "call",
            OPCODE::ret => "ret",
            OPCODE::iadd => "iadd",
            OPCODE::isub => "isub",
            OPCODE::imul => "imul",
            OPCODE::idiv => "idiv",
            OPCODE::sete => "sete",
            OPCODE::setne => "setne",
            OPCODE::setl => "setl",
            OPCODE::setg => "setg",
            OPCODE::setle => "setle",
            OPCODE::setge => "setge",
            OPCODE::imulq => "imulq",
            OPCODE::lea => "lea",
            OPCODE::setb => "setb",
            OPCODE::cqo => "cqo",
            OPCODE::leave => "leave",
            OPCODE::movsx => "movsx",
            OPCODE::EPILOGUE => unreachable!("should never try to to_string an epilogue"),
        }
    }
}

fn o_to_string(o: Option<isize>) -> String {
    match o {
        Some(o) => {
            if o > 0 {
                let mut s = String::from("+ ");
                s.push_str(&o.to_string());
                return s;
            } else {
                let mut s = String::from("- ");
                s.push_str(&(-1 * o).to_string());
                return s;
            }
        }
        None => "".to_string(),
    }
}

fn s_to_string(s: Option<isize>) -> String {
    match s {
        Some(scalar) => {
            let mut s = String::from("* ");
            s.push_str(&scalar.to_string());
            s
        }
        None => "".to_string(),
    }
}

pub fn ins_to_string(ins: ins) -> String {
    if let (OPCODE::LABEL, l1, _) = ins {
        format!("{}:", l1)
    } else {
        match (&ins.1, &ins.2) {
            (loc::Null, _) => format!("\t{} {}", ins.0.to_string(), ins.2),
            (_, loc::Null) => format!("\t{} {}", ins.0.to_string(), ins.1),
            _ => format!("\t{} {}, {}", ins.0.to_string(), ins.1, ins.2),
        }
    }
}

fn remove_whites(s: String) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

impl fmt::Display for loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            loc::Register(x) => write!(f, "{}", x.to_string()),
            loc::Literal(l) => write!(f, "{}", l),
            loc::Null => write!(f, ""),
            loc::Deref(b, i, s, o) => {
                let mut deref_str = String::new();
                match b {
                    Some(b) => match i {
                        Some(i) => {
                            deref_str.push_str(&format!(
                                "[{} + {} {} {}]",
                                b.to_string(),
                                i.to_string(),
                                s_to_string(*s),
                                o_to_string(*o)
                            ));
                        }
                        None => {
                            deref_str.push_str(&format!("[{} {}]", b.to_string(), o_to_string(*o)));
                        }
                    },
                    None => match i {
                        Some(i) => {
                            deref_str.push_str(&format!(
                                "[{} {} {}]",
                                i.to_string(),
                                s_to_string(*s),
                                o_to_string(*o)
                            ));
                        }
                        None => {}
                    },
                }
                write!(f, "QWORD PTR {}", remove_whites(deref_str))
            }
            loc::Global(g) => write!(f, "{}[rip]", g),
            loc::Label(l) => write!(f, "{}", l),
        }
    }
}

///LIRNODE
#[derive(Debug, Clone)]
pub enum LIRNode {
    Move {
        idx: usize,
        val: (Box<LIRNode>, Box<LIRNode>),
    },
    Call {
        idx: usize,
        val: (String, Vec<LIRNode>, usize),
    },
    Jump {
        idx: usize,
        val: String,
    },
    CJump {
        idx: usize,
        val: (Box<LIRNode>, String),
    },
    Return {
        idx: usize,
        val: Vec<LIRNode>,
    },
    Label {
        idx: usize,
        val: String,
    },
    Const {
        idx: usize,
        val: i64,
    },
    Temp {
        idx: usize,
        val: String,
    },
    SSA_Temp {
        idx: usize,
        val: (String, usize),
    },
    Op {
        idx: usize,
        val: (Op, Box<LIRNode>, Box<LIRNode>),
    },
    Mem {
        idx: usize,
        val: Box<LIRNode>,
    },
    Name {
        idx: usize,
        val: String,
    },
}

fn find_best_translation(
    translations: &Vec<translationfn>,
    node: &LIRNode,
    memo: &mut Vec<Option<Translation>>,
    ttable: &TTable,
    vartable: &mut HashMap<String, Register>,
) {
    if memo.get(node.get_idx()).unwrap().is_none() {
        let translation = translations
            .iter()
            .map(|x| x(node, memo, ttable, vartable))
            .filter_map(|x| x)
            .min_by_key(|x| x.cost)
            .unwrap();
        //extra unwrap to error if translation is not complete, can delete later
        memo[node.get_idx()] = Some(translation);
    };
}

impl LIRNode {
    pub fn translate(
        &self,
        memo: &mut Vec<Option<Translation>>,
        ttable: &TTable,
        vartable: &mut HashMap<String, Register>,
    ) {
        let ttable_entry = match self {
            LIRNode::Move { .. } => &ttable.mov,
            LIRNode::Call { .. } => &ttable.call,
            LIRNode::Jump { .. } => &ttable.jump,
            LIRNode::CJump { .. } => &ttable.cjump,
            LIRNode::Return { .. } => &ttable.ret,
            LIRNode::Label { .. } => &ttable.label,
            LIRNode::Op { .. } => &ttable.op,
            LIRNode::Mem { .. } => &ttable.mem,
            LIRNode::Name { .. } => &ttable.name,
            LIRNode::Const { .. } | LIRNode::Temp { .. } => &ttable.leaf,
            LIRNode::SSA_Temp { .. } => {
                unreachable!("SSA_Phi should be removed before translation")
            }
        };
        find_best_translation(ttable_entry, self, memo, ttable, vartable);
    }
    pub fn get_idx(&self) -> usize {
        match self {
            LIRNode::Move { idx, .. } => idx.clone(),
            LIRNode::Call { idx, .. } => idx.clone(),
            LIRNode::Jump { idx, .. } => idx.clone(),
            LIRNode::CJump { idx, .. } => idx.clone(),
            LIRNode::Return { idx, .. } => idx.clone(),
            LIRNode::Label { idx, .. } => idx.clone(),
            LIRNode::Const { idx, .. } => idx.clone(),
            LIRNode::Temp { idx, .. } => idx.clone(),
            LIRNode::Op { idx, .. } => idx.clone(),
            LIRNode::Mem { idx, .. } => idx.clone(),
            LIRNode::Name { idx, .. } => idx.clone(),
            LIRNode::SSA_Temp { .. } => {
                unreachable!("SSA_Phi should be removed before translation")
            }
        }
    }
}

impl Hash for LIRNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_idx().hash(state);
    }
}

impl LIRStmt {
    pub fn convert(&self) -> LIRNode {
        match self {
            LIRStmt::Move(d, e) => LIRNode::Move {
                idx: next_int(),
                val: (Box::new(d.convert()), Box::new(e.convert())),
            },
            LIRStmt::Call(e, vec, args) => LIRNode::Call {
                idx: next_int(),
                val: (
                    match e.convert() {
                        LIRNode::Name { val, .. } => val,
                        _ => unreachable!("jumps should only be created using lirexpr::name"),
                    },
                    vec.into_iter().map(|x| x.convert()).collect(),
                    args.clone(),
                ),
            },
            LIRStmt::Jump(e) => LIRNode::Jump {
                idx: next_int(),
                val: match e.convert() {
                    LIRNode::Name { val, .. } => val,
                    _ => unreachable!("jumps should only be created using lirexpr::name"),
                },
            },
            LIRStmt::CJump(_, _, _) => {
                unreachable!("two branch cjumps should have already been removed")
            }
            LIRStmt::Label(s) => LIRNode::Label {
                idx: next_int(),
                val: s.clone(),
            },
            LIRStmt::Return(vec) => LIRNode::Return {
                idx: next_int(),
                val: vec.into_iter().map(|x| x.convert()).collect(),
            },
            LIRStmt::CJump2(e, label) => LIRNode::CJump {
                idx: next_int(),
                val: (Box::new(e.convert()), label.clone()),
            },
        }
    }
}

impl LIRExpr {
    pub fn convert(&self) -> LIRNode {
        match self {
            LIRExpr::Const(n) => LIRNode::Const {
                idx: next_int(),
                val: n.clone(),
            },
            LIRExpr::Temp(str) => LIRNode::Temp {
                idx: next_int(),
                val: str.clone(),
            },
            LIRExpr::Op(op, l, r) => LIRNode::Op {
                idx: next_int(),
                val: (op.clone(), Box::new(l.convert()), Box::new(r.convert())),
            },
            LIRExpr::Mem(e) => {
                let val = e.convert();
                match val {
                    LIRNode::Name { .. } => val,
                    _ => LIRNode::Mem {
                        idx: next_int(),
                        val: Box::new(e.convert()),
                    },
                }
            }
            LIRExpr::Name(str) => LIRNode::Name {
                idx: next_int(),
                val: str.clone(),
            },
        }
    }
}

impl LIRDest {
    pub fn convert(&self) -> LIRNode {
        match self {
            LIRDest::Temp(s) => LIRNode::Temp {
                idx: next_int(),
                val: s.clone(),
            },
            LIRDest::Mem(e) => {
                let val = e.convert();
                match val {
                    LIRNode::Name { .. } => val,
                    _ => LIRNode::Mem {
                        idx: next_int(),
                        val: Box::new(e.convert()),
                    },
                }
            }
        }
    }
}
