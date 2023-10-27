use super::var_type::VarType;
use super::{AstNode, Term, Type};
use std::collections::HashMap;
use std::error::Error;

/// InterfaceFn is a type that represents an interface function in Eta
#[derive(Debug, Clone)]
pub struct InterfaceFn {
    pub identifier: String,
    pub args: Vec<(String, VarType)>,
    pub returns: Option<Vec<VarType>>,
    pub line_col: (usize, usize),
}

impl AstNode for InterfaceFn {
    fn children(&self) -> Vec<&dyn AstNode> {
        match &self.returns {
            None => vec![&self.args],
            Some(returns) => vec![&self.args, returns],
        }
    }

    fn to_string(&self) -> String {
        let arg_str = self.args.pp();
        let returns_str = match &self.returns {
            None => "".to_string(),
            Some(returns) => returns.pp(),
        };

        format!("{} ({}) ({})", self.identifier, arg_str, returns_str,)
    }

    fn pp(&self) -> String {
        match &self.returns {
            None => format!("{} ({}) () \n", self.identifier, self.args.pp(),),
            Some(returns) => format!("{} {} {} \n", self.identifier, self.args.pp(), returns.pp(),),
        }
    }

    fn type_check(&self, _gamma: &mut HashMap<String, Type>, rec: String) -> Result<Type, Box<dyn Error>> {
        Ok(self.as_type())
    }
}

impl InterfaceFn {
    fn as_type(&self) -> Type {
        match &self.returns {
            Some(ret) => {
                let mut types = ret.iter();
                let first = types.next().unwrap().clone();
                let extras = Some(types.cloned().collect());
                Type {
                    first,
                    extras,
                    term: Term::Zero,
                }
            }
            None => Type::new(VarType::Unit),
        }
    }
}
