use super::declaration::{Declaration, Use};
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// Interface is a type that represents an interface in Eta
#[derive(Debug, Clone)]
pub struct Interface {
    pub uses: Vec<Use>,
    pub decls: Vec<Declaration>,
    pub line_col: (usize, usize),
}

impl AstNode for Interface {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![&self.decls]
    }

    fn to_string(&self) -> String {
        let declarations = format!(
            "({})",
            self.decls
                .iter()
                .map(|x| format!("({}) ", x.to_string()))
                .collect::<String>()
        );

        format!("({})", declarations)
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(
        &self,
        _gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        Ok(Type::new(VarType::Unit))
    }
}
