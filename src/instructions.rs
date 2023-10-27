
// use std::mem;

// use crate::ir_types::LIRDest;
// use crate::ir_types::LIRExpr;
// use crate::ir_types::LIRStmt;
// use crate::ir_types::Op;
// use crate::translate::LIRNode;
// use crate::{get_next_temp, get_same_temp};

// pub enum base {
//     Register(u64),
//     Int(i64),
// }
// pub enum loc {
//     Deref(Option<base>, Option<base>, Option<u64>, Option<i64>), //base, index, scale, signed offset (similar to at&t syntax)
//     Literal(i64),
//     Register(u64),
//     Mem(usize),
// }
// pub type isn = (String, loc, Option<loc>);

// type isns = (Vec<isn>, u64, u64); //ins vec, register, cost

// pub trait Translate {
//     fn get_isns(&self, memo: &mut Vec<isn>) -> isns;
//     fn get_idx(&self) -> usize;
// }

// impl Translate for LIRNode {
//     fn get_isns(&self, memo: &mut Vec<isn>) -> isns {
//         match self {
//             LIRNode::Move { idx, val } => todo!(),
//             LIRNode::Call { idx, val } => todo!(),
//             LIRNode::Jump { idx, val } => todo!(),
//             LIRNode::CJump { idx, val } => todo!(),
//             LIRNode::Return { idx, val } => todo!(),
//             LIRNode::Label { val } => todo!(),
//             LIRNode::Const { val } => todo!(),
//             LIRNode::Temp { val } => todo!(),
//             LIRNode::Op { idx, val } => todo!(),
//             LIRNode::Mem { idx, val } => todo!(),
//             LIRNode::Name { val } => todo!(),
//         }
//     }
//     fn get_idx(&self) -> usize {
//         match self {
//             LIRNode::Move { idx, .. } => idx.clone(),
//             LIRNode::Call { idx, .. } => idx.clone(),
//             LIRNode::Jump { idx, .. } => idx.clone(),
//             LIRNode::CJump { idx, .. } => idx.clone(),
//             LIRNode::Return { idx, .. } => idx.clone(),
//             LIRNode::Label { .. } => 0,
//             LIRNode::Const { .. } => 0,
//             LIRNode::Temp { .. } => 0,
//             LIRNode::Op { idx, .. } => idx.clone(),
//             LIRNode::Mem { idx, .. } => idx.clone(),
//             LIRNode::Name { .. } => 0,
//         }
//     }
// }


// // converts a mem expression to mem operands of the form base + index * scale + offset
// // fn lir_to_mem_op(mem: &LIRExpr) -> loc {
// //     // LIRexprMEM
// //     if let LIRExpr::Mem(addr) = mem {
// //         match **addr {
// //             LIRExpr::Const(a) => loc::Deref(None, None, None, Some(a)),
// //             LIRExpr::Temp(r) => loc::Deref(Some(base::Register(r)), None, None, None),
// //             LIRExpr::Op(op, l, r) => {
// //                 op_to_mem_op(&op, l, r, , add)
// //             }
// //             LIRExpr::Mem(_) => todo!(),
// //             LIRExpr::Name(_) => todo!(),
// //         }
// //     }
// //     else {
// //         panic!("this function should only be called to translate mem exprs");
// //     }
// // }
