use crate::types::array::Array;
use crate::types::array_literal::TypePair;
use crate::types::expr::Expr;
use crate::types::var_type::VarType;
use std::{cell::RefCell, collections::HashMap, error::Error};

pub mod array;
pub mod array_index;
pub mod array_literal;
pub mod assignment;
pub mod declaration;
pub mod expr;
pub mod file;
pub mod function;
pub mod function_call;
pub mod if_stmt;
pub mod interface;
pub mod interface_function;
pub mod l_value;
pub mod literal;
pub mod primary;
pub mod procedure_call;
pub mod program;
pub mod record;
pub mod record_field;
pub mod statement;
pub mod var_type;
pub mod while_stmt;

fn add_indent(contents: String) -> String {
    contents.split("\n").map(|x| format!("\t{}\n", x)).collect()
}

/// Type that is returned by type checking functions
#[derive(PartialEq, Debug, Clone)]
pub struct Type {
    pub first: VarType,
    extras: Option<Vec<VarType>>,
    term: Term,
}

impl Type {
    /// Creates a new instance of type struct
    pub fn new(v: VarType) -> Self {
        Self {
            first: v,
            extras: None,
            term: Term::One,
        }
    }

    fn vartype_flatten(v: Vec<VarType>) -> Type {
        Self::flatten(v.into_iter().map(|x| Self::new(x)).collect())
    }
    /// Turns a vector of types into a single type struct
    fn flatten(v: Vec<Type>) -> Type {
        let mut t = vec![];
        for e in v {
            match e.first {
                VarType::Function(f) => {
                    for ft in f.as_type().as_vec() {
                        t.push(ft)
                    }
                }
                _ => t.push(e.first),
            }

            match e.extras {
                None => (),
                Some(extras) => {
                    for extra in extras {
                        t.push(extra);
                    }
                }
            };
        }
        let mut types = t.into_iter();

        let first = types.next().unwrap();
        let typevec = types.collect::<Vec<_>>();
        let extras = if typevec.len() == 0 {
            None
        } else {
            Some(typevec)
        };
        Type {
            first,
            extras,
            term: Term::Zero,
        }
    }

    /// Compares two types for equality
    fn same(&self, other: &Type) -> bool {
        match (&self.extras, &other.extras) {
            (None, None) => self.first == other.first,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => self.first == other.first && a == b,
        }
    }

    /// Returns a vector of types contained in the type struct
    fn as_vec(self) -> Vec<VarType> {
        match self.extras {
            None => match self.first {
                VarType::Void => vec![],
                _ => vec![self.first],
            },
            Some(extra) => std::iter::once(self.first)
                .chain(extra.into_iter())
                .collect(),
        }
    }

    /// Checks if the type is unit
    fn is_unit(&self) -> bool {
        self.first == VarType::Unit
    }

    /// Checks if the type is integer
    fn is_int(&self) -> bool {
        self.first == VarType::IntType
    }

    /// Checks if the type is boolean
    fn is_bool(&self) -> bool {
        self.first == VarType::BoolType
    }

    /// Checks if the type is a one-dimensional array
    fn is1d(&self) -> bool {
        match &self.first {
            VarType::Array(a) => {
                *a.contents == VarType::IntType || *a.contents == VarType::BoolType
            }
            _ => false,
        }
    }

    /// Checks if the return type of the statement is 0
    fn is_zero(&self) -> bool {
        self.term == Term::Zero
    }
}

/// Trait that all AST nodes must implement
pub trait AstNode {
    fn children(&self) -> Vec<&dyn AstNode>;
    fn to_string(&self) -> String;
    fn pp(&self) -> String;
    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>>;
}

/// Enum that represents the return type of a statement
#[derive(PartialEq, Debug, Clone)]
pub enum Term {
    One,
    Zero,
}

fn make_nd_array(n: usize, base_type: VarType) -> VarType {
    if n == 1 {
        VarType::Array(Array {
            size: None,
            contents: Box::new(base_type),
            line_col: (0, 0),
        })
    } else {
        VarType::Array(Array {
            size: None,
            contents: Box::new(make_nd_array(n - 1, base_type)),
            line_col: (0, 0),
        })
    }
}

/// Implementation of AstNode for vector of AstNodes
impl<T: AstNode> AstNode for Vec<T> {
    fn children(&self) -> Vec<&dyn AstNode> {
        self.into_iter().map(|x| x as &dyn AstNode).collect()
    }

    fn pp(&self) -> String {
        let str = self
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        format!("{}", str)
    }

    fn to_string(&self) -> String {
        format!(
            "({})",
            self.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    fn type_check(
        &self,
        _gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        todo!("this should never be called");
    }
}

/// type checks a vector of exprs and returns result containing the type of exprs
fn type_check_exprsb(
    gamma: &mut HashMap<String, Type>,
    rec: String,
    exprs: &Vec<Box<Expr>>,
    default: VarType,
) -> Result<Type, Box<dyn Error>> {
    if exprs.is_empty() {
        Ok(Type::new(default))
    } else {
        let mut t = vec![];
        for expr in exprs {
            t.push(expr.type_check(gamma, rec.clone())?);
        }
        Ok(Type::flatten(t))
    }
}

fn type_check_exprs(
    gamma: &mut HashMap<String, Type>,
    rec: String,
    exprs: &Vec<Expr>,
    default: VarType,
) -> Result<Type, Box<dyn Error>> {
    if exprs.is_empty() {
        Ok(Type::new(default))
    } else {
        let mut t = vec![];
        for expr in exprs {
            t.push(expr.type_check(gamma, rec.clone())?);
        }
        Ok(Type::flatten(t))
    }
}

/// binds name to vartype in the context
pub fn add_entry(
    context: &mut HashMap<String, Type>,
    name: String,
    vartype: VarType,
    line_col: (usize, usize),
    is_interface: bool,
) -> Result<(), Box<dyn Error>> {
    if let VarType::Record(r) = &vartype {
        if is_interface && context.contains_key(&name) {
            if !context.get(&name).unwrap().first.eq(&vartype) {
                return Err(format!(
                    "{}:{} error:Fields do not match for record {}",
                    line_col.0, line_col.1, name
                )
                .into());
            }
        }
    }
    if !is_interface && context.contains_key(&name) {
        // need line numbers here
        return Err(format!(
            "{}:{} error:Duplicate variable {}",
            line_col.0, line_col.1, name
        )
        .into());
    } else {
        context.insert(name, Type::new(vartype));
    }
    Ok(())
}

/// adds bindings to context
pub fn update_context(
    context: &mut HashMap<String, Type>,
    new_bindings: impl Iterator<Item = TypePair>,
    line_col: (usize, usize),
    is_interface: bool,
) -> Result<(), Box<dyn Error>> {
    for (name, vartype) in new_bindings {
        add_entry(context, name, vartype, line_col, is_interface)?;
    }
    Ok(())
}

/// removes bindings in context
fn backtrack_context<'a>(
    context: &mut HashMap<String, Type>,
    bindings: impl Iterator<Item = &'a TypePair>,
) -> Result<(), Box<dyn Error>> {
    for (name, _) in bindings {
        let _ = match context.remove(name) {
            None => unreachable!("bad very bad"),
            Some(_) => (),
        };
    }
    Ok(())
}

/// returns result containing type of id in the context gamma
fn get_id_type(
    id: &String,
    gamma: &HashMap<String, Type>,
    rec: String,
    rc: &RefCell<Option<String>>,
    line_col: (usize, usize),
) -> Result<Type, Box<dyn Error>> {
    match gamma.get(id) {
        Some(t) => {
            if let VarType::Identifier(i) = &t.first {
                let _ = rc.replace(Some(i.to_string()));
            }
            Ok(t.clone())
        }
        //need error line numbers
        None => {
            match &gamma.get(&rec) {
                Some(t) => match &t.first {
                    VarType::Record(r) => {
                        let ctype = r.fields_v.get(id).unwrap().clone();
                        if let VarType::Identifier(i) = &ctype {
                            let _ = rc.replace(Some(i.to_string()));
                        }
                        return Ok(Type::new(ctype));
                    }
                    _ => {
                        return Err(format!(
                            "{}:{} error:Name {} cannot be resolved",
                            line_col.0, line_col.1, id
                        )
                        .into())
                    }
                },
                None => {
                    return Err(format!(
                        "{}:{} error:Name {} cannot be resolved",
                        line_col.0, line_col.1, id
                    )
                    .into())
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rho_typechecker;

    #[test]
    fn test_shadow_variable_err() {
        let input =
            crate::rho_typechecker::file_to_str("tests/typecheck_errors/shadow_variable_err.eta")
                .unwrap();
        let expected =
            crate::rho_typechecker::file_to_str("tests/typecheck_errors/shadow_variable_err.typed")
                .unwrap();
        assert_eq!(
            rho_typechecker::type_check_to_string(rho_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "shadow_variable_err.eta"
            )),
            expected
        );
    }

    #[test]
    fn test_unreachable_err() {
        let input =
            crate::rho_typechecker::file_to_str("tests/typecheck_errors/unreachable_err.eta")
                .unwrap();
        let expected =
            crate::rho_typechecker::file_to_str("tests/typecheck_errors/unreachable_err.typed")
                .unwrap();
        assert_eq!(
            rho_typechecker::type_check_to_string(rho_typechecker::type_check(
                &input,
                "tests/typecheck_errors/",
                "unreachable_err.eta"
            )),
            expected
        );
    }
}
