use super::assignment::Assignment;
use super::expr::Expr;
use super::if_stmt::If;
use super::procedure_call::ProcedureCall;
use super::var_type::VarType;
use super::while_stmt::While;
use super::{AstNode, Term, Type};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Break {
    pub line_col: (usize, usize),
}

impl AstNode for Break {
    fn children(&self) -> Vec<&dyn AstNode> {
        vec![]
    }

    fn to_string(&self) -> String {
        "break".to_string()
    }

    fn pp(&self) -> String {
        "break".to_string()
    }

    fn type_check(
        &self,
        _gamma: &mut HashMap<String, Type>,
        _rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        Ok(Type::new(VarType::Unit))
    }
}

/// Statement is a type that represents a statement in Eta
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block(Vec<Box<Statement>>),
    While(While),
    If(If),
    Assignment(Assignment),
    Decl(Assignment),
    Return(Vec<Expr>),
    ProcedureCall(ProcedureCall),
    Break(Break),
}

impl AstNode for Statement {
    fn children(&self) -> Vec<&dyn AstNode> {
        match self {
            Statement::Block(stmts) => stmts.into_iter().map(|x| &**x as &dyn AstNode).collect(),
            Statement::While(while_stmt) => vec![while_stmt as &dyn AstNode],
            Statement::If(if_stmt) => vec![if_stmt as &dyn AstNode],
            Statement::Assignment(assignment) => vec![assignment as &dyn AstNode],
            Statement::Decl(assignment) => vec![assignment as &dyn AstNode],
            Statement::Return(exprs) => vec![exprs as &dyn AstNode],
            Statement::ProcedureCall(procedure_call) => vec![procedure_call as &dyn AstNode],
            Statement::Break(b) => vec![b],
        }
    }

    fn to_string(&self) -> String {
        match self {
            Statement::Block(stmts) => {
                format!(
                    "({})",
                    stmts.iter().map(|x| x.to_string()).collect::<String>()
                )
            }
            Statement::While(while_stmt) => format!("({})", while_stmt.to_string()),
            Statement::If(if_stmt) => format!("({})", if_stmt.to_string()),
            Statement::Assignment(assignment) => match assignment.rvalues {
                None => assignment.to_string(),
                Some(_) => format!("({})", assignment.to_string()),
            },
            Statement::Decl(assignment) => format!("({})", assignment.to_string()),
            Statement::Return(exprs) => {
                format!("(return {})", exprs.pp())
            }
            Statement::ProcedureCall(procedure_call) => {
                format!("({})", procedure_call.to_string())
            }
            Statement::Break(b) => {
                format!("({})", b.to_string())
            }
        }
    }

    fn pp(&self) -> String {
        match self {
            Statement::Block(stmts) => {
                super::add_indent(stmts.iter().map(|x| format!("{}\n", &x.pp())).collect())
            }
            Statement::While(while_stmt) => format!("({})", while_stmt.pp()),
            Statement::If(if_stmt) => format!("({})", if_stmt.pp()),
            Statement::Assignment(assignment) => match assignment.rvalues {
                None => assignment.pp(),
                Some(_) => format!("({})", assignment.pp()),
            },
            Statement::Decl(assignment) => format!("({})", assignment.pp()),
            Statement::Return(exprs) => {
                format!("(return {})", exprs.pp())
            }
            Statement::ProcedureCall(procedure_call) => {
                format!("({})", procedure_call.pp())
            }
            Statement::Break(b) => {
                format!("({})", b.pp())
            }
        }
    }

    fn type_check(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        match self {
            Statement::Block(_) => self.type_check_block(gamma, rec),
            Statement::While(stmt) => stmt.type_check(gamma, rec),
            Statement::If(stmt) => stmt.type_check(gamma, rec),
            Statement::Decl(stmt) => stmt.type_check(gamma, rec),
            Statement::Assignment(stmt) => stmt.type_check(gamma, rec),
            Statement::ProcedureCall(stmt) => stmt.type_check(gamma, rec),
            Statement::Return(exprs) => super::type_check_exprs(gamma, rec, exprs, VarType::Void),
            Statement::Break(b) => b.type_check(gamma, rec),
        }
    }
}

impl Statement {
    fn type_check_block(
        &self,
        gamma: &mut HashMap<String, Type>,
        rec: String,
    ) -> Result<Type, Box<dyn Error>> {
        let mut line = 0;
        let mut col = 0;
        let mut context_changes = vec![];
        let mut blocking = false;
        let mut t = Type::new(VarType::Unit);
        if let Statement::Block(stmts) = self {
            for stmt in stmts {
                if blocking {
                    return Err(format!("{}:{} error:Unreachable code", line, col).into());
                }
                let new_t = match &**stmt {
                    Statement::Assignment(assign) | Statement::Decl(assign) => {
                        stmt.type_check(gamma, rec.clone())?;
                        let new_bindings = assign.get_context_changes();
                        context_changes.extend(new_bindings.clone().into_iter());
                        super::update_context(
                            gamma,
                            new_bindings.into_iter(),
                            assign.line_col,
                            false,
                        )?;
                        Type::new(VarType::Unit)
                    }
                    Statement::If(ifstmt) => {
                        let if_t = ifstmt.type_check(gamma, rec.clone())?;
                        if if_t.is_zero() {
                            blocking = true;
                            line = ifstmt.line_col.0;
                            col = ifstmt.line_col.1;
                        };
                        if_t
                    }

                    Statement::Return(_) => {
                        blocking = true;
                        stmt.type_check(gamma, rec.clone())?
                    }
                    _ => stmt.type_check(gamma, rec.clone())?,
                };
                if !t.is_unit() && !new_t.is_unit() && !t.same(&new_t) {
                    return Err("statement type mismatch".into());
                }
                t = new_t
            }
        } else {
            unreachable!("this fn should never be called for non blocks!")
        }

        super::backtrack_context(gamma, context_changes.iter())?;
        t.term = if blocking { Term::Zero } else { Term::One };
        Ok(t)
    }
}
