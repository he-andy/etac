use super::expr::Expr;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

/// FunctionCall is a type that represents a function call in Eta
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub identifier: String,
    pub args: Vec<Box<Expr>>,
    pub rc: RefCell<Option<String>>,
    pub line_col: (usize, usize),
}

impl AstNode for FunctionCall {
    fn children(&self) -> Vec<&dyn AstNode> {
        self.args.iter().map(|x| &**x as &dyn AstNode).collect()
    }

    fn to_string(&self) -> String {
        if self.args.len() > 0 {
            let args_str = self
                .args
                .iter()
                .map(|x| (**x).to_string())
                .collect::<Vec<String>>()
                .join(" ");
            format!("({} {})", self.identifier.to_string(), args_str)
        } else {
            format!("({})", self.identifier.to_string())
        }
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        reco: String,
    ) -> Result<Type, Box<dyn Error>> {
        match self.identifier.as_str() {
            "length" => {
                if let VarType::Array(_) = (*(self.args.first().unwrap()))
                    .type_check(gamma, reco)?
                    .first
                {
                    return Ok(Type::new(VarType::IntType));
                } else {
                    return Err(format!(
                        "{}:{} error:length does not have array as arg",
                        self.line_col.0, self.line_col.1
                    )
                    .into());
                }
            }
            _ => match super::get_id_type(
                &self.identifier,
                gamma,
                reco.clone(),
                &self.rc,
                self.line_col,
            )?
            .first
            {
                VarType::Function(func) => {
                    let args = super::type_check_exprsb(
                        gamma,
                        reco,
                        &self.args,
                        VarType::Void,
                    )?
                    .as_vec();

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
                VarType::Record(rec) => {
                    let fields = super::type_check_exprsb(
                        gamma,
                        reco,
                        &self.args,
                        VarType::Void,
                    )?
                    .as_vec();
                    if rec.fields.len() != fields.len() {
                        return Err(format!(
                            "{}:{} error:Field length mismatch for records",
                            self.line_col.0, self.line_col.1
                        )
                        .into());
                    }
                    for (a, b) in rec.fields.iter().zip(fields.into_iter()) {
                        if a.1 != b {
                            return Err(format!(
                                "{}:{} error:Record args do not match record arg types",
                                self.line_col.0, self.line_col.1
                            )
                            .into());
                        }
                    }
                    Ok(rec.as_type())
                }
                _ => Err(format!("{} is not callable", self.identifier).into()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_function_call_err1() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_call_err1.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_call_err1.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "function_call_err1.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_function_call_err2() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_call_err2.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_call_err2.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "function_call_err2.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_len_error() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/len_error.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/len_error.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "len_error.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_len() {
        let input = crate::eta_typechecker::file_to_str("tests/pa3_tests/len_test.eta");
        let expected = crate::eta_typechecker::file_to_str("tests/pa3_tests/len_test.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/pa3_tests/",
                "len_test.eta"
            )),
            expected
        );
    }
}
