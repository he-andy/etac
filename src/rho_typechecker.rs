use crate::load_file;
use crate::rho_parser;
use crate::types::function::Function;
use crate::types::statement::Statement;
use crate::types::*;
use crate::types::{
    declaration::Declaration,
    declaration::Use,
    file::{File, File::Interface},
    program::Program,
    var_type::VarType,
};
use std::f32::consts::E;
use std::{collections::HashMap, collections::HashSet, error::Error};

pub fn file_to_str(filepath: &str) -> Result<String, Box<dyn Error>> {
    match load_file(&filepath) {
        Ok(contents) => Ok(contents),
        Err(_e) => Err(format!("Problem loading file: {:?}", filepath).into()),
    }
}

fn build_top_level_context(
    program: &mut Program,
    libpath: &str,
    filename: &str,
) -> Result<HashMap<String, Type>, Box<dyn Error>> {
    let module_name = String::from(&filename[..filename.len() - 3]);
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

    let mut uses = program.uses.clone();
    let mut seen = program
        .uses
        .iter()
        .map(|x| x.identifier.clone())
        .collect::<HashSet<_>>();

    // Check for interface with same name as module
    if !seen.contains(&module_name) {
        let mut module_int_file = String::from(libpath);
        module_int_file.push_str(&module_name);
        module_int_file.push_str(".ri");
        let contents = file_to_str(&module_int_file);
        if let Ok(s) = contents {
            uses.push(Use {
                identifier: module_name.clone(),
            });
            seen.insert(module_name.clone());
            if let Err(e) = type_check_interface(s, &mut g_0) {
                return Err(e);
            }
        }
    }

    let mut i: usize = 0;
    // Loop through all use statements
    while i < uses.len() {
        let import = &uses[i];
        if &import.identifier == filename {
            continue;
        }
        let mut interface_file = String::from(libpath);
        interface_file.push_str(&import.identifier);
        interface_file.push_str(".ri");
        let contents = file_to_str(&interface_file);
        if let Err(e) = contents {
            return Err(e);
        }
        match type_check_interface(contents.unwrap(), &mut g_0) {
            Ok(u) => {
                for use_statement in u {
                    if !seen.contains(&use_statement.identifier) {
                        seen.insert(use_statement.identifier.clone());
                        uses.push(use_statement);
                    }
                }
            }
            Err(e) => return Err(e),
        }
        i += 1;
    }
    program.uses = uses;
    //println!("{:?}", g_0);
    Ok(g_0)
}

//
fn type_check_interface(
    contents: String,
    g_0: &mut HashMap<String, Type>,
) -> Result<Vec<Use>, Box<dyn Error>> {
    let interface = rho_parser::parse_interface(&contents)?;
    let mut uses = vec![];
    match interface {
        Interface(interface) => {
            for u in &interface.uses {
                uses.push(Use {
                    identifier: u.identifier.clone(),
                });
            }
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

                                    if f_arg_types != func_arg_types || f.returns != func.returns {
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
                                g_0,
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
                    Declaration::GlobalDecl(decl) => update_context(
                        g_0,
                        decl.get_context_changes().into_iter(),
                        decl.line_col,
                        true,
                    )?,
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }
    return Ok(uses);
}

pub fn type_check_file(
    contents: &str,
    libpath: &str,
    filename: &str,
) -> Result<File, Box<dyn Error>> {
    let res = rho_parser::parse(contents);
    match res {
        Err(_) => {
            let s = rho_parser::parse_to_string(res);
            println!(
                "Syntax error beginning at {}:{}",
                filename,
                s.replace(" error:", ": ")
            ); //really hard-coded
            return Err(s.into());
        }
        Ok(mut file) => match file {
            File::Program(ref mut prog) => {
                let mut g_0 = build_top_level_context(prog, &libpath, &filename)?;
                match prog.type_check(&mut g_0, "".to_string()) {
                    Ok(_) => {
                        //println!("{:?}", file);
                        return Ok(file);
                    }
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
        Ok(_) => "Valid Rho Program".to_string(),
        Err(e) => e.to_string(),
    }
}
