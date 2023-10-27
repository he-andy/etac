use super::array::Array;
use super::function::Function;
use super::record::Record;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// VarType is a type that represents a variable type in Eta
#[derive(Debug, Clone)]
pub enum VarType {
    IntType,
    BoolType,
    Array(Array),
    Unit,
    Void,
    Function(Function),
    EmptyArray,
    Identifier(String),
    Record(Record),
    Null,
}

impl AstNode for VarType {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            VarType::Array(array) => vec![array],
            _ => vec![],
        }
    }

    fn to_string(&self) -> String {
        match self {
            VarType::IntType => "int".to_string(),
            VarType::BoolType => "bool".to_string(),
            VarType::Array(array) => array.to_string(),
            VarType::Unit => "()".to_string(),
            VarType::Void => "".to_string(),
            VarType::Function(function) => function.identifier.clone(),
            VarType::EmptyArray => "{}".to_string(),
            VarType::Identifier(s) => s.clone(),
            VarType::Record(r) => r.to_string(),
            VarType::Null => "null".to_string(),
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
        Ok(Type::new(self.clone()))
    }
}

impl PartialEq for VarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarType::EmptyArray, VarType::Array(_)) | (VarType::Array(_), VarType::EmptyArray) => {
                true
            }
            (VarType::IntType, VarType::IntType)
            | (VarType::BoolType, VarType::BoolType)
            | (VarType::Null, VarType::Null) => true,
            (VarType::Unit, VarType::Unit) | (VarType::Void, VarType::Void) => true,
            (VarType::Array(a), VarType::Array(b)) => a.eq(b),
            (VarType::Function(a), VarType::Function(b)) => a.eq(b),
            (VarType::Identifier(a), VarType::Identifier(b)) => a == b,
            (VarType::Identifier(_), VarType::Null) | (VarType::Null, VarType::Identifier(_)) => {
                true
            }
            (VarType::Array(_), VarType::Null) | (VarType::Null, VarType::Array(_)) => true,
            (VarType::Record(a), VarType::Record(b)) => a.eq(b),
            (_, _) => false,
        }
    }
}
