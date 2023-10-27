use crate::types::expr;

use super::expr::{Base, Expr};
use super::primary::Primary;
use super::Term;
use super::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// ArrayIndex is a type that represents an array index in Eta
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RecordField {
    Base(Box<Base>),
    Field(Box<RecordField>, Box<Expr>, usize, usize),
}

impl AstNode for RecordField {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            RecordField::Base(b) => vec![&**b],
            RecordField::Field(f, b, ..) => vec![&**f, &**b],
        }
    }

    fn to_string(&self) -> String {
        match self {
            RecordField::Base(b) => b.to_string(),
            RecordField::Field(i, e, ..) => {
                format!("([] {} {})", i.to_string(), e.to_string())
            }
        }
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        match self {
            RecordField::Base(b) => {
                if let Base::Identifier(i, rc, _) = &**b {
                    let _ = rc.replace(Some(i.to_string()));
                }
                return b.type_check(gamma, rec);
            }
            RecordField::Field(rf, e, _, _) => {
                let mut e_copy = e;
                let mut cur_type = rf.type_check(gamma, rec.clone()).unwrap().first;
                while let expr::Expr::Primary(Primary::RecordField(RecordField::Field(
                    ref b,
                    ref next_e,
                    _,
                    _,
                ))) = **e_copy
                {
                    if let RecordField::Base(bi) = &**b {
                        if let VarType::Identifier(st) = cur_type {
                            cur_type = bi.type_check(gamma, st).unwrap().first;
                        }
                        if let VarType::Identifier(ref k) = cur_type {
                            if let VarType::Record(r) = gamma.get(k).unwrap().first.clone() {
                                // cur_type = r.fields_v.get(i).unwrap().clone();
                                e_copy = next_e;
                            }
                        }
                    }
                }
                if let expr::Expr::Primary(p) = &**e_copy {
                    if let VarType::Identifier(st) = cur_type {
                        cur_type = p.type_check(gamma, st).unwrap().first;
                        return Ok(Type::new(cur_type));
                    }
                }
                unreachable!()
            }
        }
    }
}
