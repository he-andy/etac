use super::array_literal::TypePair;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub fields: Vec<TypePair>,
    pub fields_v: HashMap<String, VarType>, //field name to type (for offsets)
    pub line_col: (usize, usize),
}

impl Record {
    pub fn as_type(&self) -> Type {
        Type::new(VarType::Identifier(self.id.clone()))
    }
}

impl AstNode for Record {
    fn children(&self) -> Vec<&dyn AstNode> {
        self.fields.iter().map(|(_, t)| t as &dyn AstNode).collect()
    }

    fn to_string(&self) -> String {
        "record".to_string()
    }

    fn pp(&self) -> String {
        "record".to_string()
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        let mut seen = HashSet::new();
        for tp in &self.fields {
            if seen.contains(&tp.0) {
                return Err(format!(
                    "{}:{} error:Duplicate field name",
                    self.line_col.0, self.line_col.1
                )
                .into());
            }
            if let Err(e) = tp.type_check(gamma, rec.clone()) {
                return Err(e);
            }
            seen.insert(&tp.0);
        }
        Ok(Type::new(VarType::Record(self.clone())))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        if self.fields.len() != other.fields.len() || self.id != other.id {
            return false;
        }
        // convert to hashmaps and compare
        let self_map: HashMap<String, VarType> =
            self.fields.iter().fold(HashMap::new(), |mut acc, (k, v)| {
                acc.insert(k.to_string(), v.clone());
                acc
            });
        let other_map: HashMap<String, VarType> =
            other.fields.iter().fold(HashMap::new(), |mut acc, (k, v)| {
                acc.insert(k.to_string(), v.clone());
                acc
            });
        for (k, v) in &self_map {
            if !other_map.contains_key(k) || other_map.get(k).unwrap() != v {
                return false;
            }
        }
        return true;
    }
}
