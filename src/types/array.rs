use super::expr::Expr;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// Array is a type that represents an array in Eta
#[derive(Debug, Clone)]
pub struct Array {
    pub size: Option<Expr>,
    pub contents: Box<VarType>,
    pub line_col: (usize, usize),
}

impl AstNode for Array {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![&*self.contents]
    }

    fn to_string(&self) -> String {
        match &self.size {
            Some(size) => format!("([] {} {})", self.contents.to_string(), size.to_string()),
            None => format!("([] {})", self.contents.to_string()),
        }
    }

    fn pp(&self) -> String {
        self.to_string()
    }
    fn type_check(&self, gamma: &mut HashMap<String, Type>, rec: String) -> Result<Type, Box<dyn Error>> {
        match &*self.contents {
            VarType::Identifier(i) => {
                if !gamma.contains_key(i) {
                    // DOES NOT HAVE LINE NUMBERS
                    return Err("Type not found".into());
                }
            }
            VarType::Array(a) => return a.type_check(gamma, rec),
            _ => return Ok(Type::new(VarType::Array(self.clone())))
        }
        Ok(Type::new(VarType::Array(self.clone())))
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents
    }
}

impl Array {
    pub fn is_assignable(&self) -> Result<(), Box<dyn Error>> {
        match &*self.contents {
            VarType::IntType | VarType::BoolType => match self.size {
                None => Ok(()),
                Some(_) => Err(format!(
                    "{}:{} error:Array size already declared",
                    self.line_col.0, self.line_col.1
                )
                .into()),
            },
            VarType::Array(a) => a.is_assignable(),
            _ => unreachable!("not reachable!"),
        }
    }
}
