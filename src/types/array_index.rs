use super::expr::{Base, Expr};
use super::Term;
use super::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// ArrayIndex is a type that represents an array index in Eta
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayIndex {
    Base(Box<Base>),
    Index(Box<ArrayIndex>, Box<Expr>, usize, usize),
}

impl AstNode for ArrayIndex {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            ArrayIndex::Base(b) => vec![&**b],
            ArrayIndex::Index(i, e, ..) => vec![&**i, &**e],
        }
    }

    fn to_string(&self) -> String {
        match self {
            ArrayIndex::Base(b) => b.to_string(),
            ArrayIndex::Index(i, e, ..) => {
                format!("([] {} {})", i.to_string(), e.to_string())
            }
        }
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        match self {
            ArrayIndex::Base(b) => b.type_check(gamma, rec),
            ArrayIndex::Index(i, e, l, c) => {
                match (i.type_check(gamma, rec.clone()), e.type_check(gamma, rec)) {
                    (Ok(t1), Ok(t2)) => match (t1.first, t2.first) {
                        (VarType::Array(a), VarType::IntType) => {
                            let t1 = Type {
                                first: *a.contents,
                                extras: None,
                                term: Term::Zero,
                            };
                            Ok(t1)
                        }
                        (VarType::EmptyArray, VarType::IntType) => {
                            let t1 = Type {
                                first: VarType::EmptyArray,
                                extras: None,
                                term: Term::Zero,
                            };
                            Ok(t1)
                        }
                        (_, _) => Err(format!("{}:{} error:Invalid array index", l, c).into()),
                    },
                    (Err(error), _) => Err(error),
                    (_, Err(_error)) => match &**e {
                        Expr::Uop { line_col, .. } => Err(format!(
                            "{}:{} error:Expected int for array indexing",
                            line_col.0, line_col.1
                        )
                        .into()),
                        Expr::Bop { line_col, .. } => Err(format!(
                            "{}:{} error:Expected int for array indexing",
                            line_col.0, line_col.1
                        )
                        .into()),
                        _ => unreachable!("Should never happen"),
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_array_index() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/array_index_test.eta");
        let expected = "Valid Eta Program";
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "array_index_test.eta",
                ""
            )),
            expected
        );
    }

    #[test]
    fn test_array_index_2d() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/array_index_2d_test.eta");
        let expected = "Valid Eta Program";
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "array_index_2d_test.eta",
                ""
            )),
            expected
        );
    }

    #[test]
    fn test_array_index_2d_2() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/array_index_2d_test2.eta");
        let expected = "Valid Eta Program";
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "array_index_2d_test2.eta",
                ""
            )),
            expected
        );
    }

    #[test]
    fn test_invalid_assignment() {
        let input = crate::eta_typechecker::file_to_str("tests/pa3_tests/assign72.eta");
        let expected = crate::eta_typechecker::file_to_str("tests/pa3_tests/assign72.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "array_index_2d_test2.eta",
                ""
            )),
            expected
        );
    }
}
