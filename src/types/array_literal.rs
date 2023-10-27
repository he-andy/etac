use super::array::Array;
use super::expr::Expr;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// ArrayLiteral is a type that represents an array literal in Eta
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ArrayLiteral {
    pub inner: Vec<Box<Expr>>,
    pub line_col: (usize, usize),
}

impl AstNode for ArrayLiteral {
    fn children(&self) -> Vec<&dyn AstNode> {
        self.inner.iter().map(|x| &**x as &dyn AstNode).collect()
    }

    fn to_string(&self) -> String {
        let mut inner_str = String::new();
        inner_str.push_str("(");
        for e in &self.inner {
            inner_str.push_str(&e.to_string());
            inner_str.push_str(" ");
        }
        inner_str = inner_str.trim().to_string();
        inner_str.push_str(")");
        inner_str
    }

    fn pp(&self) -> String {
        self.to_string()
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        //special case for empty array
        if self.inner.is_empty() {
            return Ok(Type::new(VarType::EmptyArray));
        }

        let mut first_type = self.inner.first().unwrap().type_check(gamma, rec.clone())?;
        let mut iterator = self.inner.iter();

        while is_empty_arr(&first_type.first) {
            //make first_type point to the first "actual" (not-empty) type
            if let Some(x) = iterator.next() {
                first_type = x.type_check(gamma, rec.clone())?;
            } else {
                break;
            }
        }
        while let Some(child) = iterator.next() {
            if !first_type.same(&child.type_check(gamma, rec.clone())?) {
                return Err(format!(
                    "{}:{} error:Array literal does not consist of identical types",
                    self.line_col.0, self.line_col.1
                )
                .into());
            }
        }

        Ok(Type::new(VarType::Array(Array {
            size: None,
            contents: Box::new(first_type.first.clone()),
            line_col: self.line_col,
        })))
    }
}

/// dig inside the array just to check if it is a empty array
fn is_empty_arr(v: &VarType) -> bool {
    match v {
        VarType::EmptyArray => true,
        VarType::Array(a) => is_empty_arr(&a.contents),
        _ => false,
    }
}

/*
/// precondition: the Type (if any) referring to this VarType does not have extras
/// comparison of array literals
fn check_alit_eq(first: &VarType, second: &VarType) -> bool {
    match (first, second) {
        (VarType::Array(a), VarType::Array(b)) => check_alit_eq(&a.contents, &b.contents),
        (_, _) => first == second,
    }
} */

pub type TypePair = (String, VarType);

impl AstNode for TypePair {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![]
    }

    fn to_string(&self) -> String {
        let (s, v) = self;
        format!("({} {})", s, v.to_string())
    }

    fn pp(&self) -> String {
        let (s, v) = self;
        format!("{} {}", s, v.to_string())
    }
    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        if let VarType::Identifier(s) = self.1.clone() {
            if !gamma.contains_key(&s) {
                // DOES NOT HAVE LINE NUMBERS
                return Err("Type not found".into());
            }
        }
        if let VarType::Array(a) = self.1.clone() {
            return a.type_check(gamma, rec);
        }
        Ok(Type::new(self.1.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_array_error() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/array_error.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/array_error.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "array_error.eta",
                ""
            )),
            expected
        );
    }
}
