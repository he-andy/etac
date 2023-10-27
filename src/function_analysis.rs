use crate::{
    analysis, get_next_temp,
    ir_types::{Dest, HIRCompUnit, HIRExpr, HIRFuncDecl, HIRStmt, Id},
    next_int,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct FunctionAnalysis {
    name: String,
    size: usize,
    call_count: usize,
    is_recursive: bool,
}

fn expr_size(expr: &HIRExpr) -> usize {
    match expr {
        HIRExpr::Const(_) => 1,
        HIRExpr::Temp(_) => 1,
        HIRExpr::Op(_, lhs, rhs) => 1 + expr_size(lhs) + expr_size(rhs),
        HIRExpr::Mem(inner_expr) => 1 + expr_size(inner_expr),
        HIRExpr::Call(func_expr, args) => {
            1 + expr_size(func_expr) + args.iter().map(|arg| expr_size(arg)).sum::<usize>()
        }
        HIRExpr::Name(_) => 1,
        HIRExpr::ESeq(stmt, inner_expr) => 1 + stmt_size(stmt) + expr_size(inner_expr),
    }
}

fn stmt_size(stmt: &HIRStmt) -> usize {
    match stmt {
        HIRStmt::Move(dest, expr) => 1 + dest_size(dest) + expr_size(expr),
        HIRStmt::Seq(stmts) => 1 + stmts.iter().map(|s| stmt_size(s)).sum::<usize>(),
        HIRStmt::Jump(expr) => 1 + expr_size(expr),
        HIRStmt::CJump(expr, _, _) => 1 + expr_size(expr),
        HIRStmt::Call(func_expr, args, _) => {
            1 + expr_size(func_expr) + args.iter().map(|arg| expr_size(arg)).sum::<usize>()
        }
        HIRStmt::Label(_) => 1,
        HIRStmt::Return(exprs) => 1 + exprs.iter().map(|e| expr_size(e)).sum::<usize>(),
    }
}

fn dest_size(dest: &Dest) -> usize {
    match dest {
        Dest::Temp(_) => 1,
        Dest::Mem(expr) => 1 + expr_size(expr),
    }
}

fn is_recursive(stmt: &HIRStmt, func_name: &str) -> bool {
    match stmt {
        HIRStmt::Move(_, expr) => contains_recursive_call(expr, func_name),
        HIRStmt::Seq(stmts) => stmts.iter().any(|s| is_recursive(s, func_name)),
        HIRStmt::Jump(expr) => contains_recursive_call(expr, func_name),
        HIRStmt::CJump(expr, _, _) => contains_recursive_call(expr, func_name),
        HIRStmt::Call(func_expr, args, _) => {
            contains_recursive_call(func_expr, func_name)
                || args
                    .iter()
                    .any(|arg| contains_recursive_call(arg, func_name))
        }
        HIRStmt::Label(_) => false,
        HIRStmt::Return(exprs) => exprs.iter().any(|e| contains_recursive_call(e, func_name)),
    }
}

fn contains_recursive_call(expr: &HIRExpr, func_name: &str) -> bool {
    match expr {
        HIRExpr::Const(_) => false,
        HIRExpr::Temp(_) => false,
        HIRExpr::Op(_, lhs, rhs) => {
            contains_recursive_call(lhs, func_name) || contains_recursive_call(rhs, func_name)
        }
        HIRExpr::Mem(inner_expr) => contains_recursive_call(inner_expr, func_name),
        HIRExpr::Call(func_expr, args) => match &**func_expr {
            HIRExpr::Name(name) => {
                (name == func_name)
                    || args
                        .iter()
                        .any(|arg| contains_recursive_call(arg, func_name))
            }
            _ => {
                contains_recursive_call(func_expr, func_name)
                    || args
                        .iter()
                        .any(|arg| contains_recursive_call(arg, func_name))
            }
        },
        HIRExpr::Name(_) => false,
        HIRExpr::ESeq(stmt, inner_expr) => {
            is_recursive(stmt, func_name) || contains_recursive_call(inner_expr, func_name)
        }
    }
}

fn collect_call_sites(stmt: &HIRStmt, call_sites: &mut HashMap<String, usize>) {
    match stmt {
        HIRStmt::Move(_, expr) => update_call_sites(expr, call_sites),
        HIRStmt::Seq(stmts) => {
            for s in stmts {
                collect_call_sites(s, call_sites);
            }
        }
        HIRStmt::Jump(expr) => update_call_sites(expr, call_sites),
        HIRStmt::CJump(expr, _, _) => update_call_sites(expr, call_sites),
        HIRStmt::Call(func_expr, args, _) => {
            update_call_sites(func_expr, call_sites);
            for arg in args {
                update_call_sites(arg, call_sites);
            }
        }
        HIRStmt::Label(_) => (),
        HIRStmt::Return(exprs) => {
            for e in exprs {
                update_call_sites(e, call_sites);
            }
        }
    }
}

fn update_call_sites(expr: &HIRExpr, call_sites: &mut HashMap<String, usize>) {
    match expr {
        HIRExpr::Const(_) => (),
        HIRExpr::Temp(_) => (),
        HIRExpr::Op(_, lhs, rhs) => {
            update_call_sites(lhs, call_sites);
            update_call_sites(rhs, call_sites);
        }
        HIRExpr::Mem(inner_expr) => update_call_sites(inner_expr, call_sites),
        HIRExpr::Call(func_expr, args) => {
            if let HIRExpr::Name(name) = &**func_expr {
                let count = call_sites.entry(name.clone()).or_insert(0);
                *count += 1;
            }

            update_call_sites(func_expr, call_sites);
            for arg in args {
                update_call_sites(arg, call_sites);
            }
        }
        HIRExpr::Name(_) => (),
        HIRExpr::ESeq(stmt, inner_expr) => {
            collect_call_sites(stmt, call_sites);
            update_call_sites(inner_expr, call_sites);
        }
    }
}

pub fn analyze_functions(cu: &HIRCompUnit) -> Vec<FunctionAnalysis> {
    let mut analyses = vec![];

    for (_, function) in cu.functions.clone() {
        if let Some(body) = function.body {
            let mut call_sites = HashMap::new();
            collect_call_sites(&body, &mut call_sites);
            let call_count = call_sites.get(&function.name).unwrap_or(&0).clone();
            let is_recursive = is_recursive(&body, &function.name);
            let size = stmt_size(&body);
            let analysis = FunctionAnalysis {
                name: function.name,
                is_recursive,
                size,
                call_count,
            };
            analyses.push(analysis);
        }
    }

    //println!("{:?}", analyses);
    analyses
}

fn should_inline(analysis: &FunctionAnalysis) -> bool {
    // Set the threshold values based on your desired metrics
    let max_size = 10;
    let max_call_count = 2;

    // Check if the function should be inlined based on the given metrics
    analysis.size <= max_size && analysis.call_count <= max_call_count && !analysis.is_recursive
}