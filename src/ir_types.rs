use crate::get_next_temp;
use crate::types::array_index::ArrayIndex;
use crate::types::expr::Base;
use crate::types::primary::Primary;
use crate::types::record::Record;
use crate::types::record_field::RecordField;
use crate::types::{
    array, array_index, array_literal, assignment, expr, function, function_call, if_stmt, l_value,
    literal, primary, procedure_call, record_field, statement, var_type, while_stmt,
};
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

pub type Id = String;

#[derive(Debug, Clone)]
pub enum HIRExpr {
    Const(i64), // Representing all literals as integers (including Bool)
    Temp(Id),
    Op(Op, Box<HIRExpr>, Box<HIRExpr>),
    Mem(Box<HIRExpr>),
    Call(Box<HIRExpr>, Vec<Box<HIRExpr>>),
    Name(String),
    ESeq(Box<HIRStmt>, Box<HIRExpr>),
}

#[derive(Debug, Clone)]
pub enum Dest {
    Temp(Id),
    Mem(Box<HIRExpr>),
}

#[derive(Debug, Clone)]
pub enum HIRStmt {
    Move(Dest, Box<HIRExpr>),
    Seq(Vec<Box<HIRStmt>>),
    Jump(Box<HIRExpr>),
    CJump(Box<HIRExpr>, String, String),
    Call(Box<HIRExpr>, Vec<Box<HIRExpr>>, usize),
    Label(String),
    Return(Vec<Box<HIRExpr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    HMul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    LShift,
    RShift,
    Eq,
    Neq,
    Ult,
    Lt,
    Leq,
    Gt,
    Geq,
    Field,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "ADD"),
            Op::Sub => write!(f, "SUB"),
            Op::Mul => write!(f, "MUL"),
            Op::HMul => write!(f, "HMUL"),
            Op::Div => write!(f, "DIV"),
            Op::Mod => write!(f, "MOD"),
            Op::And => write!(f, "AND"),
            Op::Or => write!(f, "OR"),
            Op::Xor => write!(f, "XOR"),
            Op::LShift => write!(f, "LSHIFT"),
            Op::RShift => write!(f, "RSHIFT"),
            Op::Eq => write!(f, "EQ"),
            Op::Neq => write!(f, "NEQ"),
            Op::Ult => write!(f, "ULT"),
            Op::Lt => write!(f, "LT"),
            Op::Leq => write!(f, "LEQ"),
            Op::Gt => write!(f, "GT"),
            Op::Geq => write!(f, "GEQ"),
            Op::Field => write!(f, "FIELD"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRData {
    pub name: String,
    pub data: Vec<i64>,
}

impl fmt::Display for IRData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DATA {} (", self.name)?;
        for i in 0..self.data.len() {
            if i == 0 {
                write!(f, "{}", self.data[i])?;
            } else {
                write!(f, " {}", self.data[i])?;
            }
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, Default)]
pub struct HIRFuncDecl {
    pub name: String,
    pub n_returns: usize,
    pub body: Option<HIRStmt>,
}

#[derive(Debug, Clone)]
pub struct HIRCompUnit {
    pub name: String,
    pub functions: HashMap<String, HIRFuncDecl>,
    pub interface_functions: Vec<String>,
    pub records: HashMap<String, HashMap<String, u64>>,
    pub data_map: HashMap<String, IRData>,
    pub arrays: HashSet<Id>,
    pub out_bounds: u8,
}

#[derive(Debug, Clone)]
pub enum LIRExpr {
    Const(i64), // Representing all literals as integers (including Bool)
    Temp(Id),
    Op(Op, Box<LIRExpr>, Box<LIRExpr>),
    Mem(Box<LIRExpr>),
    Name(String),
}

impl fmt::Display for LIRExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LIRExpr::Const(i) => write!(f, "CONST {}", i),
            LIRExpr::Temp(id) => write!(f, "TEMP {}", id),
            LIRExpr::Op(op, l, r) => write!(f, "{} ({}) ({})", op, l, r),
            LIRExpr::Mem(le) => write!(f, "MEM ({})", le),
            LIRExpr::Name(s) => write!(f, "NAME {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LIRDest {
    Temp(Id),
    Mem(LIRExpr),
}

impl fmt::Display for LIRDest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LIRDest::Temp(id) => write!(f, "TEMP {}", id),
            LIRDest::Mem(le) => write!(f, "MEM ({})", le),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LIRStmt {
    Move(LIRDest, LIRExpr),
    Call(LIRExpr, Vec<LIRExpr>, usize),
    Jump(LIRExpr),
    CJump(LIRExpr, String, String),
    Label(String),
    Return(Vec<LIRExpr>),
    CJump2(LIRExpr, String),
}

impl fmt::Display for LIRStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LIRStmt::Move(ld, le) => write!(f, "MOVE ({}) ({})", ld, le),
            LIRStmt::Call(le, vec, n_returns) => {
                write!(f, "CALL_STMT {} ({})", n_returns, le)?;
                for expr in vec {
                    write!(f, " ({})", expr)?;
                }
                write!(f, "")
            }
            LIRStmt::Jump(le) => write!(f, "JUMP ({})", le),
            LIRStmt::CJump(le, lt, lf) => write!(f, "CJUMP ({}) {} {}", le, lt, lf),
            LIRStmt::Label(s) => write!(f, "LABEL {}", s),
            LIRStmt::Return(vec) => {
                write!(f, "RETURN")?;
                for expr in vec {
                    write!(f, " ({})", expr)?;
                }
                write!(f, "")
            }
            LIRStmt::CJump2(le, lt) => write!(f, "CJUMP ({}) {}", le, lt),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LIRFuncDecl {
    pub name: String,
    pub body: Vec<LIRStmt>,
    pub n_returns: usize,
}

impl fmt::Display for LIRFuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FUNC {}", self.name)?;
        if !self.body.is_empty() {
            write!(f, " (SEQ")?;
            for stmt in &self.body {
                write!(f, " ({})", stmt)?;
            }
            write!(f, ")")?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Clone)]
pub struct LIRCompUnit {
    pub name: String,
    pub functions: HashMap<String, LIRFuncDecl>,
    pub interface_functions: Vec<String>,
    pub data_map: HashMap<String, IRData>,
}

impl fmt::Display for LIRCompUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(COMPUNIT {}", self.name)?;
        for (_, data) in &self.data_map {
            write!(f, " ({})", data)?;
        }
        for (_, fd) in &self.functions {
            write!(f, " ({})", fd)?;
        }
        write!(f, ")")
    }
}

fn escape_escapes(s: String) -> String {
    let backslash_replaced = s.replace(r"\\", r"\");
    let re_escapes = Regex::new(r#"\\(?P<esc>[\\'"nt])"#).unwrap();
    let escaped_non_uni = re_escapes
        .replace_all(&backslash_replaced, |caps: &Captures| {
            let esc = &caps["esc"];
            let esc_as_str = esc.to_string();
            match esc {
                "n" => "\n".into(),
                "t" => "\t".into(),
                _ => esc_as_str,
            }
        })
        .to_string();

    let re = Regex::new(r"\\x\{(?P<unicode>[0-9a-fA-F]{1,6})\}").unwrap();
    re.replace_all(&escaped_non_uni, |caps: &Captures| {
        let numeric = u32::from_str_radix(&caps["unicode"], 16).unwrap();
        std::char::from_u32(numeric).unwrap().to_string()
    })
    .to_string()
}

/// IR translation of an AST string literal
pub fn trans_string_literal(s: String, cu: &mut HIRCompUnit) -> HIRExpr {
    // if cu.data_map.contains_key(&s) {
    //     HIRExpr::Name(cu.data_map.get(&s).unwrap().name.clone())
    // } else {
    let data_vec = escape_escapes(s)
        .chars()
        .map(|x| {
            Box::new(expr::Expr::Primary(primary::Primary::Literal(
                literal::Literal::Int(x as i64),
            )))
        })
        .collect();
    let arr_lit = array_literal::ArrayLiteral {
        inner: data_vec,
        line_col: Default::default(),
    };
    trans_array_literal(arr_lit, cu)
    /*
    let data = IRData {
        name: format!("_string_const{}", cu.data_map.len()),
        data: data_vec,
    };
    cu.data_map.insert(s_cloned.clone(), data);
    let t1 = get_next_temp("t");
    HIRExpr::ESeq(
        Box::new(HIRStmt::Move(
            Dest::Temp(t1.clone()),
            Box::new(HIRExpr::Name(
                cu.data_map.get(&s_cloned).unwrap().name.clone(),
            )),
        )),
        Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(t1)),
            Box::new(HIRExpr::Const(8)),
        )),
    )*/
}

/// IR translation of an AST literal
pub fn trans_literal(l: literal::Literal, cu: &mut HIRCompUnit) -> HIRExpr {
    match l {
        literal::Literal::Bool(b) => {
            if b {
                HIRExpr::Const(1)
            } else {
                HIRExpr::Const(0)
            }
        }
        literal::Literal::Int(i) => HIRExpr::Const(i),
        literal::Literal::Char(c) => HIRExpr::Const(c as i64),
        literal::Literal::String(s) => trans_string_literal(s, cu),
        literal::Literal::Null => HIRExpr::Mem(Box::new(HIRExpr::Const(0))),
    }
}

/// IR translation of an AST identifier
pub fn trans_identifier(s: String, cu: &mut HIRCompUnit) -> HIRExpr {
    match cu.data_map.get(&s) {
        None => HIRExpr::Temp(s),
        Some(idata) => HIRExpr::Mem(Box::new(HIRExpr::Name(idata.name.clone()))),
    }
}

///Convert an HIRExpr to a Dest type
/// Precondition: [he] can only be of values Temp and Mem
pub fn hir_expr_to_dest(he: HIRExpr) -> Dest {
    match he {
        HIRExpr::Mem(e) => Dest::Mem(e),
        HIRExpr::Temp(id) => Dest::Temp(id),
        _ => unreachable!(),
    }
}

/// helper function to build a sequence for array index operations
pub fn build_arr_idx_seq(
    ai: array_index::ArrayIndex,
    expr: expr::Expr,
    cu: &mut HIRCompUnit,
    ta: &String,
    ti: &String,
) -> Vec<Box<HIRStmt>> {
    let mut seq = vec![];
    let lok = get_next_temp("lok");
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(ta.clone()),
        Box::new(trans_array_index(ai, cu)),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(ti.clone()),
        Box::new(trans_expr(expr, cu)),
    )));
    seq.push(Box::new(HIRStmt::CJump(
        Box::new(HIRExpr::Op(
            Op::Ult,
            Box::new(HIRExpr::Temp(ti.into())),
            Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
                Op::Sub,
                Box::new(HIRExpr::Temp(ta.into())),
                Box::new(HIRExpr::Const(8)),
            )))),
        )),
        lok.clone(),
        "__eta_out_of_bounds".to_string(),
    )));
    cu.out_bounds = if cu.out_bounds <= 1 { 1 } else { 2 };
    seq.push(Box::new(HIRStmt::Label(lok)));
    seq
}

/// IR translation of AST array index
pub fn trans_array_index(ai: array_index::ArrayIndex, cu: &mut HIRCompUnit) -> HIRExpr {
    match ai {
        array_index::ArrayIndex::Index(rai, expr, ..) => {
            let (ta, ti) = (get_next_temp("t"), get_next_temp("t"));
            HIRExpr::ESeq(
                Box::new(HIRStmt::Seq(build_arr_idx_seq(*rai, *expr, cu, &ta, &ti))),
                Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
                    Op::Add,
                    Box::new(HIRExpr::Temp(ta)),
                    Box::new(HIRExpr::Op(
                        Op::Mul,
                        Box::new(HIRExpr::Temp(ti)),
                        Box::new(HIRExpr::Const(8)),
                    )),
                )))),
            )
        }
        array_index::ArrayIndex::Base(b) => match *b {
            expr::Base::Identifier(s, _, _) => trans_identifier(s, cu),
            expr::Base::StringLiteral(s) => trans_string_literal(s, cu),
            expr::Base::ArrayLiteral(al) => trans_array_literal(al, cu),
            expr::Base::FunctionCall(fc) => trans_function_call(fc, cu),
            expr::Base::Expr(e) => trans_expr(e, cu),
            expr::Base::ArrayIndex(ai) => trans_array_index(ai, cu),
        },
    }
}

/// IR translation of AST array literal
pub fn trans_array_literal(al: array_literal::ArrayLiteral, cu: &mut HIRCompUnit) -> HIRExpr {
    let mut seq = vec![];
    let tm = get_next_temp("t");
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tm.clone()),
        Box::new(HIRExpr::Call(
            Box::new(HIRExpr::Name("_eta_alloc".to_string())),
            vec![Box::new(HIRExpr::Const(al.inner.len() as i64 * 8 + 8))],
        )),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Mem(Box::new(HIRExpr::Temp(tm.clone()))),
        Box::new(HIRExpr::Const(al.inner.len() as i64)),
    )));
    for i in 0..al.inner.len() {
        seq.push(Box::new(HIRStmt::Move(
            Dest::Mem(Box::new(HIRExpr::Op(
                Op::Add,
                Box::new(HIRExpr::Temp(tm.clone())),
                Box::new(HIRExpr::Const(8 * (i + 1) as i64)),
            ))),
            Box::new(trans_expr(*al.inner[i].clone(), cu)),
        )));
    }
    HIRExpr::ESeq(
        Box::new(HIRStmt::Seq(seq)),
        Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tm)),
            Box::new(HIRExpr::Const(8)),
        )),
    )
}

pub fn find_identifier(expr: Primary) -> String {
    if let Primary::Identifier(s, _, _) = expr {
        return s;
    }
    if let Primary::ArrayIndex(ArrayIndex::Base(b)) = &expr {
        if let expr::Base::Identifier(s, _, _) = &**b {
            return s.to_string();
        }
    }
    if let Primary::ArrayIndex(ArrayIndex::Index(ai, ref exp, _, _)) = &expr {
        if let ArrayIndex::Base(b) = &**ai {
            if let Base::Identifier(s, _, _) = &**b {
                return s.to_string();
            }
        }
    }
    if let Primary::RecordField(RecordField::Field(rf, exp, _, _)) = expr {
        if let RecordField::Base(b) = *rf {
            if let Base::Identifier(s, _, _) = *b {
                return s;
            }
        }
    }
    unreachable!()
}

pub fn trans_record_field(rf: record_field::RecordField, cu: &mut HIRCompUnit) -> HIRExpr {
    match rf {
        record_field::RecordField::Field(rf, expr, ..) => {
            if let expr::Expr::Primary(p) = *expr.clone() {
                if let RecordField::Base(b) = *rf.clone() {
                    if let expr::Base::Identifier(_, rc, _) = *b {
                        let offset = cu
                            .records
                            .get(&rc.take().unwrap())
                            .unwrap()
                            .get(&find_identifier(p.clone()))
                            .unwrap();
                        if let expr::Expr::Primary(Primary::RecordField(r)) = *expr {
                            return HIRExpr::Mem(Box::new(HIRExpr::Op(
                                Op::Add,
                                Box::new(trans_record_field(r, &mut cu.clone())),
                                Box::new(HIRExpr::Const(*offset as i64)),
                            )));
                        }
                        let ret = HIRExpr::Mem(Box::new(HIRExpr::Op(
                            Op::Add,
                            Box::new(trans_record_field(*rf, &mut cu.clone())),
                            Box::new(HIRExpr::Const(*offset as i64)),
                        )));
                        if let Primary::ArrayIndex(ai) = p {
                            let tz = get_next_temp("t");
                            let mut seq = vec![];
                            seq.push(Box::new(HIRStmt::Move(
                                Dest::Temp(tz.clone()),
                                Box::new(ret),
                            )));
                            return HIRExpr::ESeq(
                                Box::new(HIRStmt::Seq(seq)),
                                Box::new(trans_array_index_record(ai, cu, tz)),
                            );
                        }
                        return ret;
                    } else {
                        unreachable!()
                    }
                }
            }
            unreachable!()
        }
        record_field::RecordField::Base(b) => match *b {
            expr::Base::Identifier(s, _, _) => trans_identifier(s, cu),
            expr::Base::StringLiteral(s) => trans_string_literal(s, cu),
            expr::Base::ArrayLiteral(al) => trans_array_literal(al, cu),
            expr::Base::FunctionCall(fc) => trans_function_call(fc, cu),
            expr::Base::Expr(e) => trans_expr(e, cu),
            expr::Base::ArrayIndex(ai) => trans_array_index(ai, cu),
        },
    }
}

/// IR translation of an AST primary
pub fn trans_primary(p: primary::Primary, cu: &mut HIRCompUnit) -> HIRExpr {
    match p {
        primary::Primary::Literal(l) => trans_literal(l, cu),
        primary::Primary::Identifier(s, _, _) => trans_identifier(s, cu),
        primary::Primary::ArrayIndex(ai) => trans_array_index(ai, cu),
        primary::Primary::ArrayLiteral(al) => trans_array_literal(al, cu),
        primary::Primary::FunctionCall(fc) => trans_function_call(fc, cu),
        primary::Primary::RecordField(rf) => trans_record_field(rf, cu),
    }
}

/// IR translation of adding arrays
pub fn trans_add_arrays(left: expr::Expr, right: expr::Expr, cu: &mut HIRCompUnit) -> HIRExpr {
    let (l, r) = (trans_expr(left, cu), trans_expr(right, cu));
    let mut seq = vec![];
    let (tl, tr, ts, tm) = (
        get_next_temp("tl"),
        get_next_temp("tr"),
        get_next_temp("ts"),
        get_next_temp("tm"),
    );
    let (tla, tra) = (get_next_temp("tla"), get_next_temp("tra"));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tla.clone()),
        Box::new(l.clone()),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tra.clone()),
        Box::new(r.clone()),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tl.clone()),
        Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
            Op::Sub,
            Box::new(HIRExpr::Temp(tla.clone())),
            Box::new(HIRExpr::Const(8)),
        )))),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tr.clone()),
        Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
            Op::Sub,
            Box::new(HIRExpr::Temp(tra.clone())),
            Box::new(HIRExpr::Const(8)),
        )))),
    )));

    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(ts.clone()),
        Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tl.clone())),
            Box::new(HIRExpr::Temp(tr.clone())),
        )),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tm.clone()),
        Box::new(HIRExpr::Call(
            Box::new(HIRExpr::Name("_eta_alloc".to_string())),
            vec![Box::new(HIRExpr::Op(
                Op::Add,
                Box::new(HIRExpr::Const(8)),
                Box::new(HIRExpr::Op(
                    Op::Mul,
                    Box::new(HIRExpr::Const(8)),
                    Box::new(HIRExpr::Temp(ts.clone())),
                )),
            ))],
        )),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Mem(Box::new(HIRExpr::Temp(tm.clone()))),
        Box::new(HIRExpr::Temp(ts.clone())),
    )));
    let (tc, lh) = (get_next_temp("tc"), get_next_temp("lh"));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tc.clone()),
        Box::new(HIRExpr::Const(1)),
    )));
    seq.push(Box::new(HIRStmt::Label(lh.clone())));
    let (lt, lf) = (get_next_temp("lt"), get_next_temp("lf"));
    seq.push(Box::new(HIRStmt::CJump(
        Box::new(HIRExpr::Op(
            Op::Neq,
            Box::new(HIRExpr::Temp(tc.clone())),
            Box::new(HIRExpr::Op(
                Op::Add,
                Box::new(HIRExpr::Temp(tl.clone())),
                Box::new(HIRExpr::Const(1)),
            )),
        )),
        lt.clone(),
        lf.clone(),
    )));
    seq.push(Box::new(HIRStmt::Label(lt)));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Mem(Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tm.clone())),
            Box::new(HIRExpr::Op(
                Op::Mul,
                Box::new(HIRExpr::Temp(tc.clone())),
                Box::new(HIRExpr::Const(8)),
            )),
        ))),
        Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Op(
                Op::Sub,
                Box::new(HIRExpr::Temp(tla)),
                Box::new(HIRExpr::Const(8)),
            )),
            Box::new(HIRExpr::Op(
                Op::Mul,
                Box::new(HIRExpr::Temp(tc.clone())),
                Box::new(HIRExpr::Const(8)),
            )),
        )))),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tc.clone()),
        Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tc.clone())),
            Box::new(HIRExpr::Const(1)),
        )),
    )));
    seq.push(Box::new(HIRStmt::Jump(Box::new(HIRExpr::Name(lh.clone())))));
    seq.push(Box::new(HIRStmt::Label(lf)));

    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tc.clone()),
        Box::new(HIRExpr::Const(1)),
    )));
    let (lhh, ltt, lff) = (
        get_next_temp("lhh"),
        get_next_temp("ltt"),
        get_next_temp("lff"),
    );
    seq.push(Box::new(HIRStmt::Label(lhh.clone())));
    seq.push(Box::new(HIRStmt::CJump(
        Box::new(HIRExpr::Op(
            Op::Neq,
            Box::new(HIRExpr::Temp(tc.clone())),
            Box::new(HIRExpr::Op(
                Op::Add,
                Box::new(HIRExpr::Temp(tr.clone())),
                Box::new(HIRExpr::Const(1)),
            )),
        )),
        ltt.clone(),
        lff.clone(),
    )));
    seq.push(Box::new(HIRStmt::Label(ltt.clone())));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Mem(Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tm.clone())),
            Box::new(HIRExpr::Op(
                Op::Mul,
                Box::new(HIRExpr::Op(
                    Op::Add,
                    Box::new(HIRExpr::Temp(tl.clone())),
                    Box::new(HIRExpr::Temp(tc.clone())),
                )),
                Box::new(HIRExpr::Const(8)),
            )),
        ))),
        Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Op(
                Op::Sub,
                Box::new(HIRExpr::Temp(tra)),
                Box::new(HIRExpr::Const(8)),
            )),
            Box::new(HIRExpr::Op(
                Op::Mul,
                Box::new(HIRExpr::Temp(tc.clone())),
                Box::new(HIRExpr::Const(8)),
            )),
        )))),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tc.clone()),
        Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tc)),
            Box::new(HIRExpr::Const(1)),
        )),
    )));
    seq.push(Box::new(HIRStmt::Jump(Box::new(HIRExpr::Name(lhh)))));
    seq.push(Box::new(HIRStmt::Label(lff)));
    HIRExpr::ESeq(
        Box::new(HIRStmt::Seq(seq)),
        Box::new(HIRExpr::Op(
            Op::Add,
            Box::new(HIRExpr::Temp(tm)),
            Box::new(HIRExpr::Const(8)),
        )),
    )
}

/// IR translation of an AST expression (curly E)
pub fn trans_expr(e: expr::Expr, cu: &mut HIRCompUnit) -> HIRExpr {
    match e.clone() {
        expr::Expr::Primary(p) => trans_primary(p, cu),
        expr::Expr::Uop { op, expr, .. } => match op {
            expr::Uop::Neg => HIRExpr::Op(
                Op::Sub,
                Box::new(HIRExpr::Const(0)),
                Box::new(trans_expr(*expr, cu)),
            ),
            expr::Uop::Not => HIRExpr::Op(
                Op::Xor,
                Box::new(trans_expr(*expr, cu)),
                Box::new(HIRExpr::Const(1)),
            ),
        },
        expr::Expr::Bop {
            op,
            left,
            right,
            line_col: _,
            is_array,
        } => {
            let ir_op = match op {
                expr::Bop::Mult => Op::Mul,
                expr::Bop::HMult => Op::HMul,
                expr::Bop::Div => Op::Div,
                expr::Bop::Rem => Op::Mod,
                expr::Bop::Add => {
                    if is_array.borrow().clone() {
                        return trans_add_arrays(*left, *right, cu);
                    } else {
                        Op::Add
                    }
                }
                expr::Bop::Sub => Op::Sub,
                expr::Bop::Lt => Op::Lt,
                expr::Bop::Leq => Op::Leq,
                expr::Bop::Gt => Op::Gt,
                expr::Bop::Geq => Op::Geq,
                expr::Bop::Eq => Op::Eq,
                expr::Bop::Neq => Op::Neq,
                expr::Bop::And | expr::Bop::Or => {
                    let (lt, lf, end) = (
                        get_next_temp("lt"),
                        get_next_temp("lf"),
                        get_next_temp("end"),
                    );
                    let t = get_next_temp("t");
                    let mut seq = vec![];
                    seq.push(Box::new(trans_control(e, lt.clone(), lf.clone(), cu)));
                    seq.push(Box::new(HIRStmt::Label(lt)));
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(t.clone()),
                        Box::new(HIRExpr::Const(1)),
                    )));
                    seq.push(Box::new(HIRStmt::Jump(Box::new(HIRExpr::Name(
                        end.clone(),
                    )))));
                    seq.push(Box::new(HIRStmt::Label(lf)));
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(t.clone()),
                        Box::new(HIRExpr::Const(0)),
                    )));
                    seq.push(Box::new(HIRStmt::Label(end)));
                    return HIRExpr::ESeq(Box::new(HIRStmt::Seq(seq)), Box::new(HIRExpr::Temp(t)));
                }
            };

            HIRExpr::Op(
                ir_op,
                Box::new(trans_expr(*left, cu)),
                Box::new(trans_expr(*right, cu)),
            )
        }
    }
}

pub fn trans_record(r_fields: Vec<Box<HIRExpr>>, cu: &mut HIRCompUnit) -> HIRExpr {
    let mut seq = vec![];
    let tm = get_next_temp("t");
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tm.clone()),
        Box::new(HIRExpr::Call(
            Box::new(HIRExpr::Name("_eta_alloc".to_string())),
            vec![Box::new(HIRExpr::Const(r_fields.len() as i64 * 8))],
        )),
    )));
    for i in 0..r_fields.len() {
        seq.push(Box::new(HIRStmt::Move(
            Dest::Mem(Box::new(HIRExpr::Op(
                Op::Add,
                Box::new(HIRExpr::Temp(tm.clone())),
                Box::new(HIRExpr::Const(8 * i as i64)),
            ))),
            Box::new(*r_fields[i].clone()),
        )));
    }
    HIRExpr::ESeq(Box::new(HIRStmt::Seq(seq)), Box::new(HIRExpr::Temp(tm)))
}

pub fn trans_array_index_record(
    ai: array_index::ArrayIndex,
    cu: &mut HIRCompUnit,
    tz: String,
) -> HIRExpr {
    match ai {
        array_index::ArrayIndex::Index(rai, expr, ..) => {
            let ti = get_next_temp("t");
            HIRExpr::ESeq(
                Box::new(HIRStmt::Seq(build_arr_idx_seq(*rai, *expr, cu, &tz, &ti))),
                Box::new(HIRExpr::Mem(Box::new(HIRExpr::Op(
                    Op::Add,
                    Box::new(HIRExpr::Temp(tz)),
                    Box::new(HIRExpr::Op(
                        Op::Mul,
                        Box::new(HIRExpr::Temp(ti)),
                        Box::new(HIRExpr::Const(8)),
                    )),
                )))),
            )
        }
        array_index::ArrayIndex::Base(b) => match *b {
            expr::Base::Identifier(s, _, _) => trans_identifier(s, cu),
            expr::Base::StringLiteral(s) => trans_string_literal(s, cu),
            expr::Base::ArrayLiteral(al) => trans_array_literal(al, cu),
            expr::Base::FunctionCall(fc) => trans_function_call(fc, cu),
            expr::Base::Expr(e) => trans_expr(e, cu),
            expr::Base::ArrayIndex(ai) => trans_array_index(ai, cu),
        },
    }
}

/// IR translation of an AST function call
pub fn trans_function_call(fc: function_call::FunctionCall, cu: &mut HIRCompUnit) -> HIRExpr {
    let fn_args: Vec<Box<HIRExpr>> = fc
        .args
        .into_iter()
        .map(|e| Box::new(trans_expr(*e, cu)))
        .collect();
    if &fc.identifier == "length" {
        return HIRExpr::Mem(Box::new(HIRExpr::Op(
            Op::Sub,
            Box::new(*fn_args[0].clone()),
            Box::new(HIRExpr::Const(8)),
        )));
    }
    if cu.records.contains_key(&fc.identifier) {
        return trans_record(fn_args, cu);
    }
    let fn_name = cu.functions.get(&fc.identifier).unwrap().name.clone();
    HIRExpr::Call(Box::new(HIRExpr::Name(fn_name)), fn_args)
}

/// IR translation of an AST function definition
pub fn trans_function_defn(
    fd: function::Function,
    cu: &mut HIRCompUnit,
    opt: bool,
) -> Result<HIRStmt, Box<dyn Error>> {
    let mut seq = vec![];
    seq.push(Box::new(HIRStmt::Label(fd.identifier)));
    let mut name_ctr = 1;
    for arg in fd.args {
        seq.push(Box::new(HIRStmt::Move(
            Dest::Temp(arg.0),
            Box::new(HIRExpr::Temp(format!("_ARG{name_ctr}"))),
        )));
        name_ctr += 1
    }

    seq.push(Box::new(trans_stmt(fd.block.clone(), cu, "".to_string())));
    if let (None, statement::Statement::Block(v)) = (fd.returns, fd.block) {
        if let Some(b) = v.last() {
            if let statement::Statement::Return(_) = **b {
            } else {
                seq.push(Box::new(HIRStmt::Return(vec![])));
            }
        }
    }
    if cu.out_bounds == 1 {
        seq.push(Box::new(HIRStmt::Label("__eta_out_of_bounds".to_string())));
        seq.push(Box::new(HIRStmt::Call(
            Box::new(HIRExpr::Name("_eta_out_of_bounds".to_string())),
            vec![],
            0,
        )));
        cu.out_bounds = 2;
    }
    if opt {
        fold_stmt(HIRStmt::Seq(seq))
    } else {
        Ok(HIRStmt::Seq(seq))
    }
}

/// IR translation of an AST procedure call
pub fn trans_procedure_call(pc: procedure_call::ProcedureCall, cu: &mut HIRCompUnit) -> HIRStmt {
    let fn_name = cu.functions.get(&pc.identifier).unwrap().name.clone();
    let fn_args = pc
        .args
        .into_iter()
        .map(|e| Box::new(trans_expr(e, cu)))
        .collect();
    HIRStmt::Call(Box::new(HIRExpr::Name(fn_name)), fn_args, 0)
}

/// IR translation of an AST function call with multiple returns
pub fn trans_function_call_mult(fc: function_call::FunctionCall, cu: &mut HIRCompUnit) -> HIRStmt {
    let func_data = cu.functions.get(&fc.identifier).unwrap().clone();
    let fn_name = &func_data.name;
    let fn_args = fc
        .args
        .into_iter()
        .map(|e| Box::new(trans_expr(*e, cu)))
        .collect();
    HIRStmt::Call(
        Box::new(HIRExpr::Name(fn_name.to_string())),
        fn_args,
        func_data.n_returns,
    )
}

/// IR translation of an AST if statement
pub fn trans_if(i: if_stmt::If, cu: &mut HIRCompUnit, bl: String) -> HIRStmt {
    let mut seq = vec![];
    let lt = get_next_temp("lt");
    let lf = get_next_temp("lf");
    let end = get_next_temp("endif");
    seq.push(Box::new(trans_control(
        i.condition,
        lt.clone(),
        lf.clone(),
        cu,
    )));
    seq.push(Box::new(HIRStmt::Label(lt)));
    seq.push(Box::new(trans_stmt(*i.stmt, cu, bl.clone())));
    if i.el != None {
        seq.push(Box::new(HIRStmt::Jump(Box::new(HIRExpr::Name(
            end.clone(),
        )))))
    }
    seq.push(Box::new(HIRStmt::Label(lf)));
    match i.el {
        None => (),
        Some(e) => {
            seq.push(Box::new(trans_stmt(*e, cu, bl.to_string())));
            seq.push(Box::new(HIRStmt::Label(end)));
        }
    };

    HIRStmt::Seq(seq)
}

/// IR translation of an AST while loop
pub fn trans_while(w: while_stmt::While, cu: &mut HIRCompUnit) -> HIRStmt {
    let mut seq = vec![];
    let lt = get_next_temp("lt");
    let lf = get_next_temp("lf");
    let lh = get_next_temp("lh");
    seq.push(Box::new(HIRStmt::Label(lh.clone())));
    seq.push(Box::new(trans_control(
        w.condition,
        lt.clone(),
        lf.clone(),
        cu,
    )));
    seq.push(Box::new(HIRStmt::Label(lt)));
    seq.push(Box::new(trans_stmt(*w.stmt, cu, lf.clone())));
    seq.push(Box::new(HIRStmt::Jump(Box::new(HIRExpr::Name(lh)))));
    seq.push(Box::new(HIRStmt::Label(lf)));
    HIRStmt::Seq(seq)
}

/// IR translation of an array (helper)
/// PRECONDITION: [a] has a size field of Some value (or this will panic)
pub fn trans_array(
    a: array::Array,
    mut sizes: Vec<String>,
    cu: &mut HIRCompUnit,
) -> (HIRExpr, Vec<Box<HIRStmt>>) {
    let mut seq = vec![];
    let (tn, tm) = (sizes.remove(0), get_next_temp("t"));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Temp(tm.clone()),
        Box::new(HIRExpr::Call(
            Box::new(HIRExpr::Name("_eta_alloc".to_string())),
            vec![Box::new(HIRExpr::Op(
                Op::Add,
                Box::new(HIRExpr::Const(8)),
                Box::new(HIRExpr::Op(
                    Op::Mul,
                    Box::new(HIRExpr::Const(8)),
                    Box::new(HIRExpr::Temp(tn.clone())),
                )),
            ))],
        )),
    )));
    seq.push(Box::new(HIRStmt::Move(
        Dest::Mem(Box::new(HIRExpr::Temp(tm.clone()))),
        Box::new(HIRExpr::Temp(tn.clone())),
    )));
    if let var_type::VarType::Array(a_inner) = *a.contents {
        if a_inner.size.is_some() {
            let (tc, lh, lt, lf) = (
                get_next_temp("tc"),
                get_next_temp("lh"),
                get_next_temp("lt"),
                get_next_temp("lf"),
            );
            seq.push(Box::new(HIRStmt::Move(
                Dest::Temp(tc.clone()),
                Box::new(HIRExpr::Const(1)),
            )));
            seq.push(Box::new(HIRStmt::Label(lh.clone())));
            seq.push(Box::new(HIRStmt::CJump(
                Box::new(HIRExpr::Op(
                    Op::Neq,
                    Box::new(HIRExpr::Temp(tc.clone())),
                    Box::new(HIRExpr::Op(
                        Op::Add,
                        Box::new(HIRExpr::Temp(tn)),
                        Box::new(HIRExpr::Const(1)),
                    )),
                )),
                lt.clone(),
                lf.clone(),
            )));
            seq.push(Box::new(HIRStmt::Label(lt.clone())));
            let (t_next, seq_next) = trans_array(a_inner, sizes, cu);
            seq.extend(seq_next);
            seq.push(Box::new(HIRStmt::Move(
                Dest::Mem(Box::new(HIRExpr::Op(
                    Op::Add,
                    Box::new(HIRExpr::Temp(tm.clone())),
                    Box::new(HIRExpr::Op(
                        Op::Mul,
                        Box::new(HIRExpr::Temp(tc.clone())),
                        Box::new(HIRExpr::Const(8)),
                    )),
                ))),
                Box::new(HIRExpr::Op(
                    Op::Add,
                    Box::new(t_next),
                    Box::new(HIRExpr::Const(8)),
                )),
            )));
            seq.push(Box::new(HIRStmt::Move(
                Dest::Temp(tc.clone()),
                Box::new(HIRExpr::Op(
                    Op::Add,
                    Box::new(HIRExpr::Temp(tc)),
                    Box::new(HIRExpr::Const(1)),
                )),
            )));
            seq.push(Box::new(HIRStmt::Jump(Box::new(HIRExpr::Name(lh)))));
            seq.push(Box::new(HIRStmt::Label(lf)));
        }
    }
    return (HIRExpr::Temp(tm), seq);
}

/// Helper function to move value in [rval] to [lval]
pub fn single_assignment(
    lval: l_value::LValue,
    rval: Box<HIRExpr>,
    cu: &mut HIRCompUnit,
) -> Option<Box<HIRStmt>> {
    match lval {
        l_value::LValue::Identifier(s, _, _) | l_value::LValue::TypePair((s, _), _) => {
            Some(Box::new(HIRStmt::Move(
                hir_expr_to_dest(trans_identifier(s, cu)),
                rval,
            )))
        }
        l_value::LValue::ArrayIndex(ai, _) => match ai {
            array_index::ArrayIndex::Index(rai, expr, ..) => {
                let (ta, ti) = (get_next_temp("ta"), get_next_temp("ti"));
                let mut seq = build_arr_idx_seq(*rai, *expr, cu, &ta, &ti);
                seq.push(Box::new(HIRStmt::Move(
                    Dest::Mem(Box::new(HIRExpr::Op(
                        Op::Add,
                        Box::new(HIRExpr::Temp(ta)),
                        Box::new(HIRExpr::Op(
                            Op::Mul,
                            Box::new(HIRExpr::Temp(ti)),
                            Box::new(HIRExpr::Const(8)),
                        )),
                    ))),
                    rval,
                )));
                Some(Box::new(HIRStmt::Seq(seq)))
            }
            _ => unreachable!(),
        },
        _ => None,
    }
}

/// IR translation of an AST declaration
pub fn trans_declaration(lvals: Vec<l_value::LValue>, cu: &mut HIRCompUnit) -> HIRStmt {
    HIRStmt::Seq(
        lvals
            .into_iter()
            .filter_map(|lval| match lval {
                l_value::LValue::TypePair((s, var_type::VarType::Array(mut a)), _) => {
                    match a.size {
                        None => {
                            let tn = get_next_temp("tn");
                            let mut seq = vec![];
                            seq.push(Box::new(HIRStmt::Move(
                                Dest::Temp(tn.clone()),
                                Box::new(HIRExpr::Mem(Box::new(HIRExpr::Const(0)))),
                            )));
                            seq.push(Box::new(HIRStmt::Move(
                                Dest::Temp(s),
                                Box::new(HIRExpr::Temp(tn.clone())),
                            )));
                            Some(Box::new(HIRStmt::Seq(seq)))
                        }
                        Some(_) => {
                            let a_copy = a.clone();
                            let tn = get_next_temp("tn");
                            let mut seq = vec![Box::new(HIRStmt::Move(
                                Dest::Temp(tn.clone()),
                                Box::new(trans_expr(a.size.clone().unwrap(), cu)),
                            ))];
                            let mut sizes = vec![tn];
                            while let var_type::VarType::Array(a_inner) = *a.contents.clone() {
                                match a_inner.size.clone() {
                                    None => break,
                                    Some(s) => {
                                        let tn = get_next_temp("tn");
                                        seq.push(Box::new(HIRStmt::Move(
                                            Dest::Temp(tn.clone()),
                                            Box::new(trans_expr(s, cu)),
                                        )));
                                        sizes.push(tn);
                                        a = a_inner;
                                    }
                                }
                            }
                            let (t, seq2) = trans_array(a_copy, sizes, cu);
                            seq.extend(seq2);
                            seq.push(Box::new(HIRStmt::Move(
                                Dest::Temp(s),
                                Box::new(HIRExpr::Op(
                                    Op::Add,
                                    Box::new(HIRExpr::Const(8)),
                                    Box::new(t),
                                )),
                            )));
                            Some(Box::new(HIRStmt::Seq(seq)))
                        }
                    }
                }
                l_value::LValue::TypePair((s, var_type::VarType::IntType), _) => {
                    let tn = get_next_temp("tn");
                    let mut seq = vec![];
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(tn.clone()),
                        Box::new(HIRExpr::Const(0)),
                    )));
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(s),
                        Box::new(HIRExpr::Temp(tn.clone())),
                    )));
                    Some(Box::new(HIRStmt::Seq(seq)))
                }
                l_value::LValue::TypePair((s, var_type::VarType::BoolType), _) => {
                    let tn = get_next_temp("tn");
                    let mut seq = vec![];
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(tn.clone()),
                        Box::new(HIRExpr::Const(0)),
                    )));
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(s),
                        Box::new(HIRExpr::Temp(tn.clone())),
                    )));
                    Some(Box::new(HIRStmt::Seq(seq)))
                }
                l_value::LValue::TypePair((s, var_type::VarType::Identifier(_)), _) => {
                    let tn = get_next_temp("tn");
                    let mut seq = vec![];
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(tn.clone()),
                        Box::new(HIRExpr::Mem(Box::new(HIRExpr::Const(0)))),
                    )));
                    seq.push(Box::new(HIRStmt::Move(
                        Dest::Temp(s),
                        Box::new(HIRExpr::Temp(tn.clone())),
                    )));
                    Some(Box::new(HIRStmt::Seq(seq)))
                }
                _ => None,
            })
            .collect(),
    )
}

/// IR translation of an AST assignment
pub fn trans_assignment(a: assignment::Assignment, cu: &mut HIRCompUnit) -> HIRStmt {
    //let mut seq = vec![];
    match a.rvalues {
        // Declaration
        None => trans_declaration(a.lvalues, cu),
        // Assignment
        Some(exprs) => {
            let mut rvals = vec![];
            let mut ass_seq = vec![];
            let lsize = a.lvalues.len();
            let lvals: Vec<Option<HIRExpr>> =
                a.lvalues
                    .into_iter()
                    .map(|lval| match lval {
                        l_value::LValue::Identifier(s, _, _)
                        | l_value::LValue::TypePair((s, _), _) => Some(trans_identifier(s, cu)),
                        l_value::LValue::ArrayIndex(ai, _) => match ai {
                            array_index::ArrayIndex::Index(rai, expr, ..) => {
                                let (ta, ti) = (get_next_temp("ta"), get_next_temp("ti"));
                                let seq = build_arr_idx_seq(*rai, *expr, cu, &ta, &ti);
                                ass_seq.extend(seq);
                                let tm = get_next_temp("tm");
                                ass_seq.push(Box::new(HIRStmt::Move(
                                    Dest::Temp(tm.clone()),
                                    Box::new(HIRExpr::Op(
                                        Op::Add,
                                        Box::new(HIRExpr::Temp(ta)),
                                        Box::new(HIRExpr::Op(
                                            Op::Mul,
                                            Box::new(HIRExpr::Temp(ti)),
                                            Box::new(HIRExpr::Const(8)),
                                        )),
                                    )),
                                )));
                                Some(HIRExpr::Mem(Box::new(HIRExpr::Temp(tm))))
                            }
                            _ => unreachable!(),
                        },
                        l_value::LValue::RecordField(rf, _) => Some(trans_record_field(rf, cu)),
                        _ => None,
                    })
                    .collect();
            if lsize != exprs.len() {
                for i in 0..lsize {
                    rvals.push(Box::new(HIRExpr::Temp(format!("_RV{}", i + 1))));
                }
                match &exprs[0] {
                    expr::Expr::Primary(primary::Primary::FunctionCall(fc)) => {
                        ass_seq.push(Box::new(trans_function_call_mult(fc.clone(), cu)));
                        ass_seq.extend(lvals.into_iter().zip(rvals.into_iter()).filter_map(
                            |(lval, rval)| match lval {
                                Some(e) => Some(Box::new(HIRStmt::Move(hir_expr_to_dest(e), rval))),
                                None => None,
                            },
                        ))
                    }
                    _ => unreachable!(),
                }
            } else {
                if lvals.len() == 1 {
                    if !lvals[0].is_none() {
                        match &exprs[0] {
                            expr::Expr::Primary(primary::Primary::FunctionCall(fc)) => {
                                ass_seq.push(Box::new(HIRStmt::Move(
                                    hir_expr_to_dest(lvals.into_iter().nth(0).unwrap().unwrap()),
                                    Box::new(trans_function_call(fc.clone(), cu)),
                                )));
                            }
                            _ => {
                                ass_seq.push(Box::new(HIRStmt::Move(
                                    hir_expr_to_dest(lvals.into_iter().nth(0).unwrap().unwrap()),
                                    Box::new(trans_expr(exprs[0].clone(), cu)),
                                )));
                            }
                        }
                    }
                } else {
                    rvals = exprs
                        .iter()
                        .map(|a| {
                            let ta = get_next_temp("ta");
                            match &a {
                                expr::Expr::Primary(primary::Primary::FunctionCall(fc)) => {
                                    ass_seq
                                        .push(Box::new(trans_function_call_mult(fc.clone(), cu)));
                                    ass_seq.push(Box::new(HIRStmt::Move(
                                        Dest::Temp(ta.clone()),
                                        Box::new(HIRExpr::Temp("_RV1".to_string())),
                                    )));
                                }
                                _ => {
                                    ass_seq.push(Box::new(HIRStmt::Move(
                                        Dest::Temp(ta.clone()),
                                        Box::new(trans_expr(a.clone(), cu)),
                                    )));
                                }
                            }
                            Box::new(HIRExpr::Temp(ta))
                        })
                        .collect();
                    for i in 0..rvals.len() {
                        match lvals[i].clone() {
                            Some(e) => ass_seq.push(Box::new(HIRStmt::Move(
                                hir_expr_to_dest(e),
                                rvals[i].clone(),
                            ))),
                            None => (),
                        }
                    }
                }
            }
            HIRStmt::Seq(ass_seq)
        } // Assignment
    }
}

fn trans_break(b: statement::Break, cu: &mut HIRCompUnit, bl: String) -> HIRStmt {
    HIRStmt::Label(bl.to_string())
}

/// IR translation of an AST statement (curly S)
pub fn trans_stmt(s: statement::Statement, cu: &mut HIRCompUnit, bl: String) -> HIRStmt {
    match s {
        statement::Statement::Block(b) => HIRStmt::Seq(
            b.into_iter()
                .map(|s| Box::new(trans_stmt(*s, cu, bl.clone())))
                .collect(),
        ),
        statement::Statement::While(w) => trans_while(w, cu),
        statement::Statement::If(i) => trans_if(i, cu, bl.clone()),
        statement::Statement::Assignment(a) => trans_assignment(a, cu),
        statement::Statement::Decl(a) => trans_assignment(a, cu),
        statement::Statement::Return(r) => {
            HIRStmt::Return(r.into_iter().map(|e| Box::new(trans_expr(e, cu))).collect())
        }
        statement::Statement::ProcedureCall(pc) => trans_procedure_call(pc, cu),
        statement::Statement::Break(b) => trans_break(b, cu, bl.clone()),
    }
}

/// (curly C): IR statemnts that has the side effects of [e], then jumps to
/// label [lt] if e evaluates to true, or to label [lf] if e evaluates to false
pub fn trans_control(e: expr::Expr, lt: String, lf: String, cu: &mut HIRCompUnit) -> HIRStmt {
    match &e {
        expr::Expr::Primary(p) => match p.clone() {
            primary::Primary::Literal(l) => match l {
                literal::Literal::Bool(b) => {
                    if b {
                        HIRStmt::Jump(Box::new(HIRExpr::Name(lt)))
                    } else {
                        HIRStmt::Jump(Box::new(HIRExpr::Name(lf)))
                    }
                }
                _ => unreachable!(),
            },
            _ => HIRStmt::CJump(Box::new(trans_primary(p.clone(), cu)), lt, lf),
        },
        expr::Expr::Bop {
            op, left, right, ..
        } => match op {
            expr::Bop::And => {
                let l1 = get_next_temp("l1");
                let mut seq = vec![];
                seq.push(Box::new(trans_control(
                    (**left).clone(),
                    l1.clone(),
                    lf.clone(),
                    cu,
                )));
                seq.push(Box::new(HIRStmt::Label(l1)));
                seq.push(Box::new(trans_control((**right).clone(), lt, lf, cu)));
                HIRStmt::Seq(seq)
            }
            expr::Bop::Or => {
                let mut seq = vec![];
                let l1 = get_next_temp("l1");
                seq.push(Box::new(trans_control(
                    (**left).clone(),
                    lt.clone(),
                    l1.clone(),
                    cu,
                )));
                seq.push(Box::new(HIRStmt::Label(l1)));
                seq.push(Box::new(trans_control((**right).clone(), lt, lf, cu)));
                HIRStmt::Seq(seq)
            }
            _ => HIRStmt::CJump(Box::new(trans_expr(e, cu)), lt, lf),
        },
        expr::Expr::Uop { op, expr, .. } => match op {
            expr::Uop::Not => HIRStmt::CJump(Box::new(trans_expr((**expr).clone(), cu)), lf, lt),
            _ => unreachable!(),
        },
    }
}

/// IR lowering for high-level IR statement
/// translates an IR statement to a sequence s1; s2; ... ;sn of canonical IR statements that have the same effect
pub fn lower_stmt(s: HIRStmt, hircu: &HIRCompUnit) -> Vec<LIRStmt> {
    match s {
        HIRStmt::Seq(s) => {
            let mut res = vec![];
            for el in s.into_iter().map(|x| lower_stmt(*x, hircu)) {
                res.extend(el);
            }
            res
        }
        HIRStmt::Jump(e) => {
            let (mut s, e) = lower_expr(*e, hircu);
            s.push(LIRStmt::Jump(e));
            s
        }
        HIRStmt::CJump(e, l1, l2) => {
            let (mut s, e) = lower_expr(*e, hircu);
            s.push(LIRStmt::CJump(e, l1, l2));
            s
        }
        HIRStmt::Label(l) => vec![LIRStmt::Label(l)],
        HIRStmt::Move(d, e) => match d {
            Dest::Temp(id) => {
                let (mut s, e) = lower_expr(*e, hircu);
                s.push(LIRStmt::Move(LIRDest::Temp(id.clone()), e.clone()));
                s
            }
            Dest::Mem(le) => match is_commutative(*le, *e, hircu) {
                (true, mut s1, s2, e1, e2) => {
                    s1.extend(s2);
                    s1.push(LIRStmt::Move(LIRDest::Mem(e1), e2));
                    s1
                }
                (false, mut s1, s2, e1, e2) => {
                    s1.push(LIRStmt::Move(LIRDest::Temp("_t".into()), e1));
                    s1.extend(s2);
                    s1.push(LIRStmt::Move(LIRDest::Mem(LIRExpr::Temp("_t".into())), e2));
                    s1
                }
            },
        },
        HIRStmt::Return(v) => {
            let mut seq = vec![];
            let mut seq2 = vec![];
            let mut ret = vec![];
            let mut i = 1;
            for expr in v {
                let (s, e) = lower_expr(*expr, hircu);
                seq.extend(s);
                seq2.push(LIRStmt::Move(LIRDest::Temp(format!("_RV{}", i)), e));
                ret.push(LIRExpr::Temp(format!("_RV{}", i)));
                i += 1;
            }
            seq.extend(seq2);
            seq.push(LIRStmt::Return(ret));
            seq
        }
        HIRStmt::Call(id, args, n_returns) => {
            let mut s_acc = vec![];
            let ef = lower_expr(*id, hircu);
            s_acc.extend(ef.0);

            let arglist = (1..(args.len() + 1))
                .map(|_| LIRExpr::Temp(get_next_temp("t")))
                .collect::<Vec<LIRExpr>>();
            for (arg, argname) in args.into_iter().zip(arglist.clone()) {
                let (s, e) = lower_expr(*arg, hircu);
                let label = match argname {
                    LIRExpr::Temp(name) => name,
                    _ => unreachable!("??xd"),
                };
                s_acc.extend(s);
                s_acc.push(LIRStmt::Move(LIRDest::Temp(label), e));
            }
            s_acc.push(LIRStmt::Call(ef.1, arglist, n_returns));
            s_acc
        }
    }
}

/// IR lowering for high-level IR expression
/// translates an IR expression to a sequence of canonical IR statements that have the same effect, and an expression that has the same value if evaluated after the whole sequence of statements.
pub fn lower_expr(e: HIRExpr, hircu: &HIRCompUnit) -> (Vec<LIRStmt>, LIRExpr) {
    match e {
        HIRExpr::Const(i) => (vec![], LIRExpr::Const(i)),
        HIRExpr::Name(l) => (vec![], LIRExpr::Name(l)),
        HIRExpr::Temp(t) => (vec![], LIRExpr::Temp(t)),
        HIRExpr::ESeq(_, _) => {
            let mut acc = vec![];
            let e = fold_eseq(&mut acc, e, hircu);
            (acc, e)
        }
        HIRExpr::Mem(e) => {
            let (s, e) = lower_expr(*e, hircu);
            (s, LIRExpr::Mem(Box::new(e)))
        }
        HIRExpr::Call(id, args) => {
            let mut s_acc = vec![];
            let ef = lower_expr(*id, hircu);
            s_acc.extend(ef.0);

            let arglist = (1..(args.len() + 1))
                .map(|_| LIRExpr::Temp(get_next_temp("t")))
                .collect::<Vec<LIRExpr>>();
            for (arg, argname) in args.into_iter().zip(arglist.clone()) {
                let (s, e) = lower_expr(*arg, hircu);
                let label = match argname {
                    LIRExpr::Temp(name) => name,
                    _ => unreachable!("??xd"),
                };
                s_acc.extend(s);
                s_acc.push(LIRStmt::Move(LIRDest::Temp(label), e));
            }
            s_acc.push(LIRStmt::Call(ef.1, arglist, 1));
            let t = get_next_temp("t");
            s_acc.push(LIRStmt::Move(
                LIRDest::Temp(t.clone()),
                LIRExpr::Temp("_RV1".into()),
            ));
            (s_acc, LIRExpr::Temp(t))
        }
        HIRExpr::Op(op, l, r) => {
            let (mut is_comm, mut s1, s2, e1, e2) = is_commutative(*l, *r, hircu);
            match &e1 {
                LIRExpr::Name(id) => {
                    for stmt in &s2 {
                        if let LIRStmt::Move(LIRDest::Mem(LIRExpr::Name(id2)), _) = stmt {
                            if id == id2 {
                                is_comm = false;
                            }
                        }
                        if let LIRStmt::Call(..) = stmt {
                            is_comm = false;
                        }
                    }
                }
                _ => (),
            }
            if is_comm {
                s1.extend(s2);
                (s1, LIRExpr::Op(op, Box::new(e1), Box::new(e2)))
            } else {
                let mut s = vec![];
                let t1 = get_next_temp("t");
                s.extend(s1);
                s.push(LIRStmt::Move(LIRDest::Temp(t1.clone()), e1));
                s.extend(s2);
                (
                    s,
                    LIRExpr::Op(op, Box::new(LIRExpr::Temp(t1)), Box::new(e2)),
                )
            }
        }
    }
}

fn fold_eseq(acc: &mut Vec<LIRStmt>, e: HIRExpr, hircu: &HIRCompUnit) -> LIRExpr {
    match e {
        HIRExpr::ESeq(s, e) => {
            acc.extend(lower_stmt(*s, hircu));
            fold_eseq(acc, *e, hircu)
        }
        _ => lower_expr(e, hircu).1,
    }
}

/// Returns if two unpure expressions are commutative (additionally returns lowered stmts/exprs) (needs extra logic for OP)
fn is_commutative(
    e1: HIRExpr,
    e2: HIRExpr,
    hircu: &HIRCompUnit,
) -> (bool, Vec<LIRStmt>, Vec<LIRStmt>, LIRExpr, LIRExpr) {
    let (s1, e1) = lower_expr(e1, hircu);
    let (s2, e2) = lower_expr(e2, hircu);
    match &e1 {
        LIRExpr::Mem(_) => {
            for stmt in &s2 {
                if let LIRStmt::Move(LIRDest::Mem(_), _) = stmt {
                    return (false, s1, s2, e1, e2);
                }
            }
            (true, s1, s2, e1, e2)
        }
        LIRExpr::Temp(id) => {
            for stmt in &s2 {
                if let LIRStmt::Move(LIRDest::Temp(id2), _) = stmt {
                    if id == id2 {
                        return (false, s1, s2, e1, e2);
                    }
                }
            }
            (true, s1, s2, e1, e2)
        }
        _ => (true, s1, s2, e1, e2),
    }
}

pub fn checked_computation(op: &Op, l: i64, r: i64) -> Result<HIRExpr, Box<dyn Error>> {
    let res = match op {
        Op::Add => l.checked_add(r),
        Op::Sub => l.checked_sub(r),
        Op::Mul => l.checked_mul(r),
        Op::Div => l.checked_div(r),
        Op::Mod => l.checked_rem(r),
        Op::RShift => {
            if r.is_negative() {
                None
            } else {
                let bounded_r = r.max(u32::MAX as i64) as u32;
                Some(l.wrapping_shr(bounded_r))
            }
        }
        Op::LShift => {
            if r.is_negative() {
                None
            } else {
                let bounded_r = r.max(u32::MAX as i64) as u32;
                Some(l.wrapping_shl(bounded_r))
            }
        }
        _ => unreachable!(
            "checked_computation should not be called for {}",
            op.to_string()
        ),
    };
    match res {
        Some(v) => Ok(HIRExpr::Const(v)),
        None => Err(format!(
            "{} between {} and {} is undefined and may potentially cause underflow or overflow",
            op.to_string(),
            l,
            r
        )
        .into()),
    }
}

pub fn fold_stmt(stmt: HIRStmt) -> Result<HIRStmt, Box<dyn Error>> {
    match stmt {
        HIRStmt::Move(d, e) => {
            let d_folded = match d {
                Dest::Mem(m) => Dest::Mem(fold_expr(*m)?.into()),
                Dest::Temp(_) => d,
            };
            let e_folded = fold_expr(*e)?;
            Ok(HIRStmt::Move(d_folded, e_folded.into()))
        }
        HIRStmt::Call(e, args, n) => {
            let e_folded = fold_expr(*e)?;
            let args_folded = args
                .into_iter()
                .map(|x| fold_expr(*x))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|x| Box::new(x))
                .collect();
            Ok(HIRStmt::Call(e_folded.into(), args_folded, n))
        }
        HIRStmt::Jump(e) => Ok(HIRStmt::Jump(fold_expr(*e)?.into())),
        HIRStmt::CJump(e, t, f) => Ok(HIRStmt::CJump(fold_expr(*e)?.into(), t, f)),
        HIRStmt::Label(_) => Ok(stmt),
        HIRStmt::Return(evec) => Ok(HIRStmt::Return(
            evec.into_iter()
                .map(|x| fold_expr(*x))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|x| Box::new(x))
                .collect(),
        )),
        HIRStmt::Seq(s) => Ok(HIRStmt::Seq(
            s.into_iter()
                .map(|x| fold_stmt(*x))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|x| Box::new(x))
                .collect(),
        )),
    }
}

pub fn fold_expr(expr: HIRExpr) -> Result<HIRExpr, Box<dyn Error>> {
    match expr {
        HIRExpr::Op(op, l, r) => {
            let l_folded = fold_expr(*l)?;
            //short circuit

            if let HIRExpr::Const(lconst) = l_folded {
                match op {
                    Op::Or => {
                        if lconst == 1 {
                            return Ok(HIRExpr::Const(1));
                        }
                    }
                    Op::And => {
                        if lconst == 0 {
                            return Ok(HIRExpr::Const(0));
                        }
                    }
                    _ => (),
                };
            }
            let r_folded = fold_expr(*r)?;
            Ok(
                if let (HIRExpr::Const(lconst), HIRExpr::Const(rconst)) =
                    (l_folded.clone(), r_folded.clone())
                {
                    match op {
                        Op::Field => todo!(),
                        Op::Xor => HIRExpr::Const(lconst ^ rconst),
                        Op::Neq => HIRExpr::Const((lconst != rconst) as i64),
                        Op::Eq => HIRExpr::Const((lconst == rconst) as i64),
                        Op::And => HIRExpr::Const(lconst & rconst),
                        Op::Or => HIRExpr::Const(lconst | rconst),
                        Op::HMul => HIRExpr::Const(i64::from_be_bytes(
                            ((lconst as i128) * (rconst as i128)).to_be_bytes()[0..8]
                                .try_into()
                                .expect("unreachable"),
                        )),
                        Op::Lt => HIRExpr::Const((lconst < rconst) as i64),
                        Op::Leq => HIRExpr::Const((lconst <= rconst) as i64),
                        Op::Gt => HIRExpr::Const((lconst > rconst) as i64),
                        Op::Geq => HIRExpr::Const((lconst >= rconst) as i64),
                        Op::Ult => {
                            let l_u64 = u64::from_be_bytes(lconst.to_be_bytes());
                            let r_u64 = u64::from_be_bytes(rconst.to_be_bytes());
                            HIRExpr::Const((l_u64 < r_u64) as i64)
                        }
                        Op::Add
                        | Op::Mul
                        | Op::Sub
                        | Op::Div
                        | Op::Mod
                        | Op::LShift
                        | Op::RShift => checked_computation(&op, lconst, rconst)?,
                    }
                } else {
                    HIRExpr::Op(op, Box::new(l_folded), Box::new(r_folded))
                },
            )
        }
        HIRExpr::Const(_) => Ok(expr),
        HIRExpr::Temp(_) => Ok(expr),
        HIRExpr::Mem(e) => Ok(HIRExpr::Mem(fold_expr(*e)?.into())),
        HIRExpr::Name(_) => Ok(expr),
        HIRExpr::Call(e, args) => {
            let e_folded = fold_expr(*e)?;
            let args_folded = args
                .into_iter()
                .map(|x| fold_expr(*x))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|x| Box::new(x))
                .collect();
            Ok(HIRExpr::Call(e_folded.into(), args_folded))
        }
        HIRExpr::ESeq(s, e) => Ok(HIRExpr::ESeq(s, fold_expr(*e)?.into())),
    }
}
