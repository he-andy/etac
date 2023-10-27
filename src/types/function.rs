use super::statement::Statement;
use super::var_type::VarType;
use super::{AstNode, Type};
use std::collections::HashMap;
use std::error::Error;

/// Function is a type that represents a function in Eta
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub identifier: String,
    pub args: Vec<(String, VarType)>,
    pub returns: Option<Vec<VarType>>,
    pub block: Statement,
    pub line_col: (usize, usize),
}

impl Function {
    pub fn as_type(&self) -> Type {
        match &self.returns {
            Some(ret) => Type::vartype_flatten(ret.clone()),
            None => Type::new(VarType::Unit),
        }
    }
}

impl AstNode for Function {
    fn children(&self) -> Vec<&dyn AstNode> {
        match &self.returns {
            None => vec![&self.args, &self.block],
            Some(returns) => vec![&self.args, &self.block, returns],
        }
    }

    fn to_string(&self) -> String {
        let arg_str = self.args.pp();
        let returns_str = match &self.returns {
            None => "".to_string(),
            Some(returns) => returns.pp(),
        };

        format!(
            "{} ({}) ({}) \n {}",
            self.identifier,
            arg_str,
            returns_str,
            self.block.to_string()
        )
    }

    fn pp(&self) -> String {
        match &self.returns {
            None => format!(
                "{} ({}) () \n({})",
                self.identifier,
                self.args.pp(),
                self.block.pp()
            ),
            Some(returns) => format!(
                "{} {} {} \n({})",
                self.identifier,
                self.args.pp(),
                returns.pp(),
                self.block.pp()
            ),
        }
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        super::update_context(gamma, self.args.clone().into_iter(), self.line_col, false)?;
        let func_type = self.as_type();
        let mut block_type = self.block.type_check(gamma, rec)?; //needs to be 0 if fn, R if procedure

        if block_type.first == VarType::Void {
            //convert block_type to unit if it is void
            block_type.first = VarType::Unit;
        }

        super::backtrack_context(gamma, self.args.iter())?;
        if self.returns != None && !block_type.is_zero() {
            return Err(format!(
                "{}:{} error:Block must return",
                self.line_col.0, self.line_col.1
            )
            .into());
        }
        if block_type.same(&func_type) {
            Ok(func_type)
        } else {
            Err(format!(
                "{}:{} error:Function body does not evaluate to correct type",
                self.line_col.0, self.line_col.1
            )
            .into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eta_typechecker;

    #[test]
    fn test_function_err1() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_err1.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_err1.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "function_err1.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_function_err2() {
        let input = crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_err2.eta");
        let expected =
            crate::eta_typechecker::file_to_str("tests/typecheck_errors/function_err2.typed");
        assert_eq!(
            eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "function_err2.eta"
            )),
            expected
        );
    }
}
