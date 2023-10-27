use super::array_literal::TypePair;
use super::expr::Expr;
use super::l_value::LValue;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

/// Assignment is a type that represents an assignment in Eta
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub lvalues: Vec<LValue>,
    pub rvalues: Option<Vec<Expr>>,
    pub line_col: (usize, usize),
}

impl Assignment {
    pub fn get_context_changes(&self) -> Vec<TypePair> {
        self.lvalues
            .iter()
            .filter(|x| match x {
                LValue::TypePair(_, _) => true,
                _ => false,
            })
            .map(|x| match x {
                LValue::TypePair(t, _) => t,
                _ => unreachable!("?"),
            })
            .cloned()
            .collect()
    }
}

impl AstNode for Assignment {
    fn children(&self) -> Vec<&dyn AstNode> {
        match &self.rvalues {
            None => vec![&self.lvalues as &dyn AstNode],
            Some(rvalues) => vec![&self.lvalues as &dyn AstNode, rvalues as &dyn AstNode],
        }
    }

    fn to_string(&self) -> String {
        let lvalue_str = if self.lvalues.len() < 2 {
            self.lvalues.pp()
        } else {
            self.lvalues.to_string()
        };
        match &self.rvalues {
            None => lvalue_str,
            Some(rvalues) => format!(
                "= {} {}",
                lvalue_str,
                rvalues.pp() // if rvalues.len() < 2 {
                             //     rvalues.pp()
                             // } else {
                             //     rvalues.to_string()
                             // }
            ),
        }
    }

    fn pp(&self) -> String {
        match &self.rvalues {
            None => self.lvalues.first().unwrap().pp(),
            Some(rvalues) => format!("{} {}", self.lvalues.first().unwrap().pp(), rvalues.pp()),
        }
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        match &self.rvalues {
            None => {
                for lval in self.lvalues.iter() {
                    if let LValue::TypePair(t, _) = lval {
                        if let VarType::Record(r) = &t.1 {
                            if let Err(e) = r.type_check(gamma, rec.clone()) {
                                return Err(e);
                            }
                        }
                        if let Err(e) = t.type_check(gamma, rec.clone()) {
                            return Err(e);
                        }
                    }
                }
                // if let LValue::TypePair(t, _) = self.lvalues.first().unwrap() {
                //     if let VarType::Record(r) = &t.1 {
                //         return r.type_check(gamma);
                //     }
                // }
                Ok(Type::new(VarType::Unit))
            }
            Some(rv) => {
                let right_t =
                    super::type_check_exprs(gamma, rec.clone(), rv, VarType::Void)?.as_vec();
                let mut left_t = vec![];
                for lvalue in &self.lvalues {
                    left_t.push(lvalue.type_check(gamma, rec.clone())?.first)
                }

                if left_t.len() != right_t.len() {
                    return Err(format!(
                        "{}:{} error:Assignment length mismatch",
                        self.line_col.0, self.line_col.1
                    )
                    .into());
                }

                for (l, r) in left_t.into_iter().zip(right_t.into_iter()) {
                    if (l != VarType::Unit && l != r && r != VarType::Null)
                        || (l == VarType::IntType && r == VarType::Null)
                        || (l == VarType::BoolType && r == VarType::Null)
                    {
                        return Err(format!(
                            "{}:{} error:Assignment type mismatch",
                            self.line_col.0, self.line_col.1
                        )
                        .into());
                    }
                }
                Ok(Type::new(VarType::Unit))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_assignment_err1() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/assignment_error1.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/assignment_error1.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "assignment_error1.eta",
                ""
            )),
            expected
        );
    }

    #[test]
    fn test_assignment_err2() {
        let input =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/assignment_error2.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/assignment_error2.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "assignment_error2.eta",
                ""
            )),
            expected
        );
    }
}
