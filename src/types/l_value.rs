use super::array_index::ArrayIndex;
use super::array_literal::TypePair;
use super::record_field::RecordField;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

type LineCol = (usize, usize);

/// LValue is a type that represents a type that can appear on the left-hand side of an assign statement in Eta
#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    Identifier(String, RefCell<Option<String>>, LineCol),
    TypePair(TypePair, LineCol),
    ArrayIndex(ArrayIndex, LineCol),
    RecordField(RecordField, LineCol),
    Discard,
}

impl AstNode for LValue {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            LValue::ArrayIndex(array_index, _) => vec![array_index],
            _ => vec![],
        }
    }

    fn to_string(&self) -> String {
        match self {
            LValue::Identifier(id, _, _) => id.to_string(),
            LValue::TypePair(tp, _) => tp.to_string(),
            LValue::ArrayIndex(ai, _) => ai.to_string(),
            LValue::Discard => "_".to_string(),
            LValue::RecordField(rf,_) => rf.to_string(),
        }
    }

    fn pp(&self) -> String {
        match self {
            LValue::Identifier(id, _, _) => id.to_string(),
            LValue::TypePair(tp, _) => tp.pp(),
            LValue::ArrayIndex(ai, _) => ai.to_string(),
            LValue::Discard => "_".to_string(),
            LValue::RecordField(rf, _) => rf.to_string(),
        }
    }

    fn type_check(&self, gamma: &mut HashMap<String, Type>, rec: String) -> Result<Type, Box<dyn Error>> {
        match &self {
            LValue::Identifier(id, rc, _) => {
                let id_type = super::get_id_type(id, gamma, rec.clone(), rc, (0, 0))?;

                match &id_type.first {
                    VarType::IntType | VarType::BoolType => Ok(id_type),
                    VarType::Array(a) => {
                        a.is_assignable()?;
                        Ok(id_type)
                    }
                    _ => Err("invalid type for lvalue".into()),
                }
            }
            LValue::TypePair(tp, _) => tp.type_check(gamma, rec),
            LValue::ArrayIndex(ai, _) => ai.type_check(gamma, rec),
            LValue::RecordField(rf, _) => rf.type_check(gamma, rec),
            LValue::Discard => Ok(Type::new(VarType::Unit)),
        }
    }
}
