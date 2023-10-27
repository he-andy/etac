use super::expr::Expr;
use super::statement::Statement;
use super::{AstNode, Term, Type};
use std::collections::HashMap;
use std::error::Error;

/// If is a type that represents an if statement in Eta
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub condition: Expr,
    pub stmt: Box<Statement>,
    pub el: Option<Box<Statement>>,
    pub line_col: (usize, usize),
}

impl AstNode for If {
    fn children(&self) -> Vec<&dyn AstNode> {
        match &self.el {
            None => vec![&self.condition as &dyn AstNode, &*self.stmt as &dyn AstNode],
            Some(el) => vec![
                &self.condition as &dyn AstNode,
                &*self.stmt as &dyn AstNode,
                &**el as &dyn AstNode,
            ],
        }
    }

    fn to_string(&self) -> String {
        match &self.el {
            None => format!(
                "if {} {}",
                self.condition.to_string(),
                self.stmt.to_string()
            ),
            Some(el) => format!(
                "if {} {} {}",
                self.condition.to_string(),
                self.stmt.to_string(),
                el.to_string()
            ),
        }
    }

    fn pp(&self) -> String {
        match &self.el {
            None => format!(
                "if {} ({})",
                self.condition.pp(),
                super::add_indent(self.stmt.pp())
            ),
            Some(el) => format!(
                "if {} ({}) ({})",
                self.condition.pp(),
                super::add_indent(self.stmt.pp()),
                super::add_indent(el.pp())
            ),
        }
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
        match &self.el {
            None => {
                stmt_type.term = Term::One;
                Ok(stmt_type)
            }
            Some(el_stmt) => {
                let el_stmt_type = el_stmt.type_check(gamma, rec)?;

                if !(stmt_type.is_unit() || el_stmt_type.is_unit() || stmt_type.same(&el_stmt_type))
                {
                    Err(format!(
                        "{}:{} error:Branches for if statement are not valid",
                        self.line_col.0, self.line_col.1
                    )
                    .into())
                } else {
                    if stmt_type.is_zero() && el_stmt_type.is_zero() {
                        Ok(stmt_type)
                    } else if stmt_type.is_zero() {
                        stmt_type.term = Term::One;
                        Ok(stmt_type)
                    } else if el_stmt_type.is_zero() {
                        stmt_type.term = Term::One;
                        Ok(stmt_type)
                    } else {
                        if !stmt_type.is_unit() {
                            Ok(stmt_type)
                        } else {
                            Ok(el_stmt_type)
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_if_err() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/if_error.eta");
        let expected = crate::eta_typechecker::file_to_str("tests/typecheck_errors/if_error.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "if_error.eta"
            )),
            expected
        );
    }
}
