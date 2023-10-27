use super::declaration::{Declaration, Use};
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// Program is a type that represents a program in Eta
#[derive(Debug, Clone)]
pub struct Program {
    pub uses: Vec<Use>,
    pub decls: Vec<Declaration>,
    pub line_col: (usize, usize),
}

impl AstNode for Program {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![&self.uses, &self.decls]
    }
    fn to_string(&self) -> String {
        let uses = if self.uses.is_empty() {
            "()".to_string()
        } else {
            format!(
                "({} )",
                self.uses
                    .iter()
                    .map(|x| format!("{} ", x.to_string()))
                    .collect::<String>()
            )
        };
        let declarations = format!(
            "({})",
            self.decls
                .iter()
                .map(|x| format!("({}) ", x.to_string()))
                .collect::<String>()
        );

        format!("({} {})", uses, declarations)
    }

    fn pp(&self) -> String {
        let uses = format!(
            "({})",
            super::add_indent(
                self.uses
                    .iter()
                    .map(|x| format!("{}\n", x.pp()))
                    .collect::<String>()
            )
        );
        let declarations = format!(
            "({})",
            super::add_indent(
                self.decls
                    .iter()
                    .map(|x| format!("({}\n) ", x.pp()))
                    .collect::<String>()
            )
        );

        format!(
            "({})",
            super::add_indent(format!("{}\n{}\n", uses, declarations))
        )
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        for decls in &self.decls {
            match decls {
                Declaration::Function(f) => f.type_check(gamma, rec.clone())?,
                Declaration::GlobalDecl(a) => a.type_check(gamma, rec.clone())?,
                _ => unreachable!(),
            };
        }

        for import in &self.uses {
            import.type_check(gamma, rec.clone())?;
        }
        Ok(Type::new(VarType::Unit))
    }
}
