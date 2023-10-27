use super::expr::Expr;
use super::statement::Statement;
use super::{AstNode, Term, Type};
use std::collections::HashMap;
use std::error::Error;

/// While is a type that represents a while statement in Eta
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub condition: Expr,
    pub stmt: Box<Statement>,
    pub line_col: (usize, usize),
}

impl AstNode for While {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![&self.condition, &*self.stmt]
    }

    fn to_string(&self) -> String {
        format!(
            "while {} {}",
            self.condition.to_string(),
            self.stmt.to_string()
        )
    }

    fn pp(&self) -> String {
        format!(
            "while {} ({})",
            self.condition.pp(),
            super::add_indent(self.stmt.pp())
        )
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        let condition_type = self.condition.type_check(gamma, rec.clone())?;
        if !condition_type.is_bool() {
            return Err(format!(
                "{}:{} error:Condition must be boolean type",
                self.line_col.0, self.line_col.1
            )
            .into());
        }

        let mut stmt_type = self.stmt.type_check(gamma, rec.clone())?;
        stmt_type.term = Term::One;
        Ok(stmt_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_while_err() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/while_err.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/while_err.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "while_err.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_while_edge_case() {
        let input = crate::eta_typechecker::file_to_str("tests/pa3_tests/while_edge_case.eta");
        let expected = crate::eta_typechecker::file_to_str("tests/pa3_tests/while_edge_case.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/pa3_tests/",
                "while_edge_case.eta"
            )),
            expected
        );
    }
}
