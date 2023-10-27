use super::{AstNode, Type};
use crate::types::expr::Expr;
use crate::types::var_type::VarType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

/// ProcedureCall is a type that represents a procedure call in Eta
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureCall {
    pub identifier: String,
    pub args: Vec<Expr>,
    pub rc: RefCell<Option<String>>,
    pub line_col: (usize, usize),
}

impl AstNode for ProcedureCall {
    fn children(&self) -> Vec<&dyn AstNode> {
        self.args.iter().map(|x| x as &dyn AstNode).collect()
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.identifier, self.args.pp())
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        match super::get_id_type(
            &self.identifier,
            gamma,
            rec.clone(),
            &self.rc,
            self.line_col,
        )?
        .first
        {
            VarType::Function(func) => {
                let args = super::type_check_exprs(gamma, rec, &self.args, VarType::Void)?.as_vec();

                if func.args.len() != args.len() {
                    return Err(format!(
                        "{}:{} error:Argument length mismatch",
                        self.line_col.0, self.line_col.1
                    )
                    .into());
                }
                for (a, b) in func.args.iter().zip(args.into_iter()) {
                    if a.1 != b {
                        return Err(format!(
                            "{}:{} error:Caller args do not match function arg types",
                            self.line_col.0, self.line_col.1
                        )
                        .into());
                    }
                }

                Ok(func.as_type())
            }
            _ => Err(format!(
                "{}:{} error:{} is not callable",
                self.line_col.0, self.line_col.1, self.identifier
            )
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_procedure_call_err1() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/procedure_call_err1.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/procedure_call_err1.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "procedure_call_err1.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_procedure_call_err2() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/procedure_call_err2.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/procedure_call_err2.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "procedure_call_err2.eta"
            )),
            expected
        );
    }
}
