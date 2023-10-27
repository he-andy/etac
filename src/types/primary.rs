use crate::types::array_index::ArrayIndex;
use crate::types::array_literal::ArrayLiteral;
use crate::types::function_call::FunctionCall;
use crate::types::get_id_type;
use crate::types::literal::Literal;
use crate::types::make_nd_array;
use crate::types::AstNode;
use crate::types::Type;
use crate::types::VarType;
use std::{collections::HashMap, error::Error};
use std::cell::RefCell;

use super::record_field::RecordField;

type LineCol = (usize, usize);

/// Primary is a type that can appear at the bottom-most level of an expression in Eta
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Primary {
    Literal(Literal),
    Identifier(String, RefCell<Option<String>>, LineCol),
    ArrayIndex(ArrayIndex),
    ArrayLiteral(ArrayLiteral),
    FunctionCall(FunctionCall),
    RecordField(RecordField),
}

impl AstNode for Primary {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            Primary::Literal(_) => vec![],
            Primary::Identifier(_, _, _) => vec![],
            Primary::ArrayIndex(array_index) => vec![array_index],
            Primary::ArrayLiteral(array_literal) => vec![array_literal],
            Primary::FunctionCall(fn_call) => vec![fn_call],
            Primary::RecordField(rf) => vec![rf],
        }
    }

    fn to_string(&self) -> String {
        match self {
            Primary::Literal(lit) => match lit {
                Literal::Bool(b) => b.to_string(),
                Literal::Int(i) => i.to_string(),
                Literal::String(s) => format!("\"{}\"", s.to_string()),
                Literal::Char(c) => format!("'{}'", c.to_string()),
                Literal::Null => "null".to_string(),
            },
            Primary::Identifier(id, _, _) => id.to_string(),
            Primary::ArrayIndex(array_index) => array_index.to_string(),
            Primary::ArrayLiteral(array_literal) => array_literal.to_string(),
            Primary::FunctionCall(fn_call) => fn_call.to_string(),
            Primary::RecordField(rf) => rf.to_string(),
        }
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(&self, gamma: &mut HashMap<String, Type>, rec: String) -> Result<Type, Box<dyn Error>> {
        match self {
            Primary::Literal(lit) => match lit {
                Literal::Bool(_) => Ok(Type::new(VarType::BoolType)),
                Literal::Int(_) => Ok(Type::new(VarType::IntType)),
                Literal::String(_) => Ok(Type::new(make_nd_array(1, VarType::IntType))),
                Literal::Char(_) => Ok(Type::new(VarType::IntType)),
                Literal::Null => Ok(Type::new(VarType::Null)),
            },
            Primary::ArrayIndex(ai) => ai.type_check(gamma, rec),
            Primary::Identifier(id, rc, lc) => get_id_type(id, gamma, rec.clone(), rc, *lc),
            Primary::ArrayLiteral(al) => al.type_check(gamma, rec),
            Primary::FunctionCall(fc) => fc.type_check(gamma, rec),
            Primary::RecordField(rf) => rf.type_check(gamma, rec),
        }
    }
}
