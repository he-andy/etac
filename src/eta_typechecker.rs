use crate::eta_parser;
use crate::load_file;
use crate::types::function::Function;
use crate::types::statement::Statement;
use crate::types::*;
use crate::types::{
    declaration::Declaration,
    file::{File, File::Interface},
    program::Program,
    var_type::VarType,
};
use std::{collections::HashMap, error::Error};

pub fn file_to_str(filepath: &str) -> String {
    match load_file(&filepath) {
        Ok(contents) => contents,
        Err(_e) => panic!("Problem loading file: {:?}", filepath),
    }
}

fn build_top_level_context(
    program: &Program,
    libpath: &str,
) -> Result<HashMap<String, Type>, Box<dyn Error>> {
    let mut g_0: HashMap<String, Type> = HashMap::new();
    //add length to context
    add_entry(
        &mut g_0,
        "length".to_string(),
        VarType::Void,
        program.line_col,
        false,
    )?;
    for decl in &program.decls {
        match decl {
            Declaration::Function(func) => add_entry(
                &mut g_0,
                func.identifier.clone(),
                VarType::Function(func.clone()),
                func.line_col,
                false,
            )?,

            Declaration::GlobalDecl(decl) => update_context(
                &mut g_0,
                decl.get_context_changes().into_iter(),
                decl.line_col,
                false,
            )?,
            Declaration::InterfaceFn(_func) => unreachable!(),
        };
    }

    for import in &program.uses {
        let mut interface_file = String::from(libpath);
        interface_file.push_str(&import.identifier);
        interface_file.push_str(".eti");
        let contents = file_to_str(&interface_file);
        let interface = eta_parser::parse_interface(&contents)?;
        match interface {
            Interface(interface) => {
                for decl in &interface.decls {
                    match decl {
                        Declaration::InterfaceFn(func) => {
                            if g_0.contains_key(&func.identifier) {
                                let func2 = &g_0.get(&func.identifier).unwrap().first;
                                match func2 {
                                    VarType::Function(f) => {
                                        let f_arg_types: Vec<VarType> =
                                            f.args.clone().into_iter().map(|x| x.1).collect();
                                        let func_arg_types: Vec<VarType> =
                                            func.args.clone().into_iter().map(|x| x.1).collect();

                                        if f_arg_types != func_arg_types
                                            || f.returns != func.returns
                                        {
                                            return Err(format!(
                                                "{}:{} error:Duplicate variable {}",
                                                f.line_col.0, f.line_col.1, f.identifier
                                            )
                                            .into());
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                add_entry(
                                    &mut g_0,
                                    func.identifier.clone(),
                                    VarType::Function(Function {
                                        identifier: func.identifier.clone(),
                                        args: func.args.clone(),
                                        returns: func.returns.clone(),
                                        block: Statement::Block(vec![]),
                                        line_col: func.line_col,
                                    }),
                                    func.line_col,
                                    true,
                                )?;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    //println!("{:?}", g_0);
    Ok(g_0)
}

pub fn type_check_file(
    contents: &str,
    libpath: &str,
    filename: &str,
) -> Result<File, Box<dyn Error>> {
    let res = eta_parser::parse(contents);
    match res {
        Err(_) => {
            let s = eta_parser::parse_to_string(res);
            println!(
                "Syntax error beginning at {}:{}",
                filename,
                s.replace(" error:", ": ")
            ); //really hard-coded
            return Err(s.into());
        }
        Ok(file) => match file {
            File::Program(ref prog) => {
                let mut g_0 = build_top_level_context(&prog, &libpath)?;
                match prog.type_check(&mut g_0, "".to_string()) {
                    Ok(_) => return Ok(file),
                    Err(e) => {
                        println!(
                            "Semantic error beginning at {}:{}",
                            filename,
                            e.to_string().replace("error:", "")
                        );
                        return Err(e);
                    }
                }
            }
            _ => todo!(),
        },
    };
}
pub fn type_check(contents: &str, libpath: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    type_check_file(contents, libpath, filename)?;
    Ok(())
}

pub fn type_check_to_string(res: Result<(), Box<dyn Error>>) -> String {
    match res {
        Ok(_) => "Valid Eta Program".to_string(),
        Err(e) => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::load_file;

    use super::*;

    fn file_to_str(filepath: &str) -> String {
        match load_file(&filepath) {
            Ok(contents) => contents,
            Err(_e) => panic!("Problem loading file: {:?}", filepath),
        }
    }

    // whitespace
    fn remove_whites(s: String) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn test_typechecker_duplicate_namespace() {
        let input = "./tests/typecheck_errors/duplicate_namespace.eta";
        let expected = "3:1error:Duplicatevariablex";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_overloaded_function() {
        let input = "./tests/typecheck_errors/overloaded_function.eta";
        let expected = "5:1error:Duplicatevariablex";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_duplicate_subscope() {
        let input = "./tests/typecheck_errors/duplicate_subscope.eta";
        let expected = "4:5error:Duplicatevariablex";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_global_scope() {
        let input = "./tests/typecheck_errors/global_scope.eta";
        let expected = "4:9error:Duplicatevariablex";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_multi_assign_var() {
        let input = "./tests/typecheck_errors/multi_assign_var.eta";
        let expected = "0:0error:Nameycannotberesolved";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_empty_array() {
        let input = "./tests/typecheck_errors/empty_array.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_statement_vs_block() {
        let input = "./tests/typecheck_errors/statement_vs_block.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_empty_list2() {
        let input = "./tests/typecheck_errors/empty_list2.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_jagged_array() {
        let input = "./tests/typecheck_errors/jagged_array.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_different_types_array() {
        let input = "./tests/typecheck_errors/different_types_array.eta";
        let expected = "2:17error:Arrayliteraldoesnotconsistofidenticaltypes";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_multiple_use() {
        let input = "./tests/typecheck_errors/multiple_use.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./tests/typecheck_errors/",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_duplicate_namespace2() {
        let input = "./tests/typecheck_errors/duplicate_namespace2.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./tests/typecheck_errors/",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_multiple_semantic() {
        let input = "./tests/typecheck_errors/multiple_semantic.eta";
        let expected = "2:13error:expectedop_plus,op_minus,op_high_multiply,op_multiply,op_divide,op_modulo,op_and,op_or,op_geq,op_gt,op_leq,op_lt,op_eq,op_neq,orop_assign";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_array_wildcard() {
        let input = "./tests/typecheck_errors/array_wildcard.eta";
        let expected = "1:11error:expectedEOI,identifier,empty_bracket_capture,orglobdecl";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
    #[test]
    fn test_typechecker_array_init3() {
        let input = "./tests/typecheck_errors/array_init3.eta";
        let expected = "Valid Eta Program";
        assert_eq!(
            remove_whites(type_check_to_string(type_check(
                &file_to_str(input),
                "./",
                ""
            ))),
            remove_whites(expected.to_string())
        );
    }
}
