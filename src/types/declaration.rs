use super::assignment::Assignment;
use super::function::Function;
use super::interface_function::InterfaceFn;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// Declaration is a type that represents a declaration in Eta
#[derive(Debug, Clone)]
pub enum Declaration {
    Function(Function),
    InterfaceFn(InterfaceFn),
    GlobalDecl(Assignment),
}

impl AstNode for Declaration {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            Declaration::Function(f) => vec![f],
            Declaration::GlobalDecl(d) => vec![d],
            Declaration::InterfaceFn(d) => vec![d],
        }
    }
    fn to_string(&self) -> String {
        match self {
            Declaration::Function(f) => format!("{}", f.to_string()),
            Declaration::GlobalDecl(d) => format!(":global {}", d.pp()),
            Declaration::InterfaceFn(d) => format!("{}", d.to_string()),
        }
    }

    fn pp(&self) -> String {
        match self {
            Declaration::Function(f) => format!("({})", f.pp()),
            Declaration::GlobalDecl(d) => format!("(:global {})", d.pp()),
            Declaration::InterfaceFn(d) => format!("({})", d.pp()),
        }
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        match self {
            Declaration::Function(f) => f.type_check(gamma, rec),
            Declaration::GlobalDecl(d) => d.type_check(gamma, rec),
            Declaration::InterfaceFn(d) => d.type_check(gamma, rec),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Use {
    pub identifier: String,
}

impl AstNode for Use {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![]
    }
    fn to_string(&self) -> String {
        format!("(use {})", self.identifier)
    }
    fn pp(&self) -> String {
        format!("(use {})", self.identifier)
    }
    fn type_check(
        &self,
        _gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        Ok(Type::new(VarType::Unit))
    }
}
