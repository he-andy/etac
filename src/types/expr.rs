use super::array_index::ArrayIndex;
use super::array_literal::{ArrayLiteral, TypePair};
use super::function_call::FunctionCall;
use super::primary::Primary;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

/// Type alias for line and colum pair
type LineCol = (usize, usize);

/// Type that represents unary operators
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Uop {
    Neg,
    Not,
}

impl Uop {
    fn to_string(&self) -> String {
        match self {
            Uop::Neg => "-".to_string(),
            Uop::Not => "!".to_string(),
        }
    }
}

/// Type that represents binary operators
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bop {
    Mult,
    HMult,
    Div,
    Rem,
    Add,
    Sub,
    Lt,
    Leq,
    Gt,
    Geq,
    Eq,
    Neq,
    And,
    Or,
}

impl Bop {
    fn to_string(&self) -> String {
        match self {
            Bop::Mult => "*".to_string(),
            Bop::HMult => "*>>".to_string(),
            Bop::Div => "/".to_string(),
            Bop::Rem => "%".to_string(),
            Bop::Add => "+".to_string(),
            Bop::Sub => "-".to_string(),
            Bop::Lt => "<".to_string(),
            Bop::Leq => "<=".to_string(),
            Bop::Gt => ">".to_string(),
            Bop::Geq => ">=".to_string(),
            Bop::Eq => "==".to_string(),
            Bop::Neq => "!=".to_string(),
            Bop::And => "&".to_string(),
            Bop::Or => "|".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Base {
    Identifier(String, RefCell<Option<String>>, LineCol),
    StringLiteral(String),
    ArrayLiteral(ArrayLiteral),
    ArrayIndex(ArrayIndex),
    FunctionCall(FunctionCall),
    Expr(Expr),
}

impl AstNode for Base {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            Base::Identifier(_, ..) => vec![],
            Base::StringLiteral(_) => vec![],
            Base::ArrayLiteral(array_literal) => vec![array_literal],
            Base::FunctionCall(_) => vec![],
            Base::Expr(expr) => vec![expr],
            Base::ArrayIndex(ai) => vec![ai],
        }
    }

    fn to_string(&self) -> String {
        match self {
            Base::Identifier(id, ..) => id.to_string(),
            Base::StringLiteral(s) => s.to_string(),
            Base::ArrayLiteral(array_literal) => array_literal.to_string(),
            Base::FunctionCall(fn_call) => fn_call.to_string(),
            Base::Expr(expr) => expr.to_string(),
            Base::ArrayIndex(ai) => ai.to_string(),
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
            Base::Identifier(id, rc, lc) => super::get_id_type(id, gamma, rec, rc, *lc),
            Base::StringLiteral(_) => Ok(Type::new(super::make_nd_array(1, VarType::IntType))),
            Base::ArrayLiteral(al) => al.type_check(gamma, rec),
            Base::FunctionCall(fc) => fc.type_check(gamma, rec),
            Base::Expr(expr) => expr.type_check(gamma, rec),
            Base::ArrayIndex(ai) => ai.type_check(gamma, rec),
        }
    }
}

/// Expr is the type that represents expressions in Eta
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Primary(Primary),
    Uop {
        op: Uop,
        expr: Box<Expr>,
        line_col: LineCol,
    },
    Bop {
        op: Bop,
        left: Box<Expr>,
        right: Box<Expr>,
        line_col: LineCol,
        is_array: RefCell<bool>,
    },
}

fn get_type_from_tpvec(s: String, vec: Vec<TypePair>) -> Option<Type> {
    for (st, v) in vec {
        if s == st {
            return Some(Type::new(v));
        }
    }
    None
}

impl AstNode for Expr {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            Expr::Primary(p) => vec![p],
            Expr::Uop {
                op: _,
                expr: e,
                line_col: _,
            } => vec![&**e],
            Expr::Bop {
                op: _,
                left: l,
                right: r,
                line_col: _,
                is_array: _,
            } => vec![&**l, &**r],
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expr::Primary(p) => p.to_string(),
            Expr::Uop {
                op,
                expr,
                line_col: _,
            } => {
                format!("({} {})", op.to_string(), expr.to_string())
            }
            Expr::Bop {
                op,
                left,
                right,
                line_col: _,
                is_array: _,
            } => {
                format!(
                    "({} {} {})",
                    op.to_string(),
                    left.to_string(),
                    right.to_string()
                )
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
            Expr::Primary(p) => p.type_check(gamma, rec),
            Expr::Uop {
                op,
                expr,
                line_col: lc,
            } => {
                let t = expr.type_check(gamma, rec)?;
                match op {
                    Uop::Not => {
                        if t.is_bool() {
                            Ok(t)
                        } else {
                            Err(format!(
                                "{}:{} error:Cannot logical negate non-boolean expression",
                                lc.0, lc.1
                            )
                            .into())
                        }
                    }
                    Uop::Neg => {
                        if t.is_int() {
                            Ok(t)
                        } else {
                            Err(format!(
                                "{}:{} error:Cannot negate non-integer expression",
                                lc.0, lc.1
                            )
                            .into())
                        }
                    }
                }
            }
            Expr::Bop {
                op,
                left,
                right,
                line_col: lc,
                is_array: a,
            } => match op {
                Bop::Neq | Bop::Eq => {
                    let l = left.type_check(gamma, rec.clone())?;
                    let r = right.type_check(gamma, rec)?;
                    if l.same(&r) {
                        Ok(Type::new(VarType::BoolType))
                    } else {
                        Err(format!(
                            "{}:{} error:Neq/Eq not defined for mismatched types",
                            lc.0, lc.1
                        )
                        .into())
                    }
                }
                Bop::Or | Bop::And => {
                    let l = left.type_check(gamma, rec.clone())?;
                    let r = right.type_check(gamma, rec)?;
                    if l.is_bool() && r.is_bool() {
                        Ok(l)
                    } else {
                        Err(format!(
                            "{}:{} error:Logical or/and not defined for non-boolean types",
                            lc.0, lc.1
                        )
                        .into())
                    }
                }
                Bop::Add => {
                    let l = left.type_check(gamma, rec.clone())?;
                    let r = right.type_check(gamma, rec)?;
                    if l.is_int() && r.is_int() || r.is1d() && l.is1d() {
                        a.replace(r.is1d());
                        Ok(l)
                    } else {
                        Err(format!(
                            "{}:{} error:+ not defined for non-int/int[] types",
                            lc.0, lc.1
                        )
                        .into())
                    }
                }
                Bop::Lt | Bop::Leq | Bop::Gt | Bop::Geq => {
                    let l = left.type_check(gamma, rec.clone())?;
                    let r = right.type_check(gamma, rec)?;
                    if l.is_int() && r.is_int() {
                        Ok(Type::new(VarType::BoolType))
                    } else {
                        Err(format!(
                            "{}:{} error:{} not defined for non-int types",
                            lc.0,
                            lc.1,
                            op.to_string()
                        )
                        .into())
                    }
                }
                _ => {
                    let l = left.type_check(gamma, rec.clone())?;
                    let r = right.type_check(gamma, rec)?;
                    if l.is_int() && r.is_int() {
                        Ok(l)
                    } else {
                        Err(format!(
                            "{}:{} error:{} not defined for non-int types",
                            lc.0,
                            lc.1,
                            op.to_string()
                        )
                        .into())
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_expr_err1() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err1.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err1.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "expr_err1.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_expr_err2() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err2.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err2.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "expr_err2.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_expr_err3() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err3.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err3.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "expr_err3.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_expr_err4() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err4.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err4.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "expr_err4.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_expr_err5() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err5.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err5.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "expr_err5.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_expr_err6() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err6.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/expr_err6.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "expr_err6.eta"
            )),
            expected
        );
    }
}
