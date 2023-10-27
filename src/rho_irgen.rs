use crate::ir_types;
use crate::ir_types::LIRCompUnit;
use crate::ir_types::LIRFuncDecl;
use crate::load_file;
use crate::reorder;
use crate::rho_parser;
use crate::translate::OptOptions;
use crate::types::array::Array;
use crate::types::file;
use crate::types::file::File;
use crate::types::var_type::VarType;
use crate::types::{
    declaration, file::File::Interface, function, interface_function, l_value, program, var_type,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn file_to_str(filepath: &str) -> String {
    match load_file(&filepath) {
        Ok(contents) => contents,
        Err(_e) => panic!("Problem loading file: {:?}", filepath),
    }
}

/// helper function to encode variable (recursively for arrays)
pub fn encode_var(v: &var_type::VarType) -> String {
    match v {
        var_type::VarType::IntType => "i".to_string(),
        var_type::VarType::BoolType => "b".to_string(),
        var_type::VarType::Array(a) => "a".to_string() + &(encode_var(&*a.contents)),
        var_type::VarType::Identifier(r) => "r".to_string() + &r.len().to_string() + &r.to_string(),
        _ => String::new(),
    }
}

/// Function to output an encoded function name based on ABI specs
pub fn encode_function_name(fd: &function::Function) -> String {
    let mut enc_name = "_I".to_string();
    enc_name.push_str(&fd.identifier.replace("_", "__"));
    enc_name.push_str("_");
    match &fd.returns {
        Some(rv) => {
            if rv.len() > 1 {
                enc_name.push_str(&format!("t{}", rv.len()));
            }
            for v in rv {
                enc_name.push_str(&encode_var(&v));
            }
        }
        None => enc_name.push_str("p"),
    };
    for (_, v) in &fd.args {
        enc_name.push_str(&encode_var(&v));
    }
    enc_name
}

/// Function to output an encoded function name based on ABI specs
pub fn encode_interface_function_name(fd: &interface_function::InterfaceFn) -> String {
    let mut enc_name = "_I".to_string();
    enc_name.push_str(&fd.identifier.replace("_", "__"));
    enc_name.push_str("_");
    match &fd.returns {
        Some(rv) => {
            if rv.len() > 1 {
                enc_name.push_str(&format!("t{}", rv.len()));
            }
            for v in rv {
                enc_name.push_str(&encode_var(&v));
            }
        }
        None => enc_name.push_str("p"),
    };
    for (_, v) in &fd.args {
        enc_name.push_str(&encode_var(&v));
    }
    enc_name
}

/// HIR translation of an Eta program
pub fn translate_program_hir(
    p: program::Program,
    name: String,
    libpath: &str,
    opt_options: &OptOptions,
) -> Result<ir_types::HIRCompUnit, Box<dyn Error>> {
    let mut cu = ir_types::HIRCompUnit {
        name: name,
        functions: HashMap::new(),
        interface_functions: vec![],
        records: HashMap::new(),
        data_map: HashMap::new(),
        arrays: HashSet::new(),
        out_bounds: 0,
    };
    for decl in &p.decls {
        match decl {
            declaration::Declaration::Function(f) => {
                let fn_decl = ir_types::HIRFuncDecl {
                    n_returns: f.returns.clone().unwrap_or_default().len(),
                    name: encode_function_name(f),
                    body: None,
                };
                cu.functions.insert(f.identifier.clone(), fn_decl);
                cu.arrays = HashSet::new();
            }
            declaration::Declaration::GlobalDecl(a) => {
                let mut var_name = String::from("");
                if let l_value::LValue::TypePair(tp, _) = a.lvalues.first().unwrap() {
                    var_name.push_str(&tp.clone().0);
                    if let VarType::Record(r) = &tp.1 {
                        let mut field_map = HashMap::new();
                        for i in 0..r.fields.len() {
                            let field_name = r.fields[i].clone().0;
                            field_map.insert(field_name, (i * 8) as u64);
                        }
                        cu.records.insert(tp.clone().0, field_map);
                        // println!("{:?}", cu.records);
                    }
                }

                let mut data = vec![];
                match &a.rvalues {
                    None => {
                        if let l_value::LValue::TypePair((_, VarType::IntType), _) =
                            a.lvalues.first().unwrap()
                        {
                            data.push(0);
                        }
                        if let l_value::LValue::TypePair((_, VarType::BoolType), _) =
                            a.lvalues.first().unwrap()
                        {
                            data.push(0);
                        }
                        //wait guys. 0 or null???
                        if let l_value::LValue::TypePair((_, VarType::Identifier(_)), _) =
                            a.lvalues.first().unwrap()
                        {
                            data.push(0);
                        }
                        if let l_value::LValue::TypePair((_, VarType::Array(a)), _) =
                            a.lvalues.first().unwrap()
                        {
                            data.push(0);
                        }
                    }
                    Some(x) => {
                        let expr = ir_types::trans_expr(x.first().unwrap().clone(), &mut cu);
                        match expr {
                            ir_types::HIRExpr::Const(l) => {
                                data.push(l);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                let data_node = ir_types::IRData {
                    name: format!("__{}", var_name.clone()),
                    data,
                };
                cu.data_map.insert(var_name, data_node);
            }
            _ => {}
        }
    }
    for import in &p.uses {
        let mut interface_file = String::from(libpath);
        interface_file.push_str(&import.identifier);
        interface_file.push_str(".ri");
        let contents = file_to_str(&interface_file);
        let interface = rho_parser::parse_interface(&contents).ok().unwrap();
        match interface {
            Interface(interface) => {
                for decl in &interface.decls {
                    match decl {
                        declaration::Declaration::InterfaceFn(f) => {
                            let fn_decl = ir_types::HIRFuncDecl {
                                name: encode_interface_function_name(f),
                                ..Default::default()
                            };
                            cu.functions.insert(f.identifier.clone(), fn_decl);
                            cu.interface_functions
                                .push(encode_interface_function_name(f));
                        }
                        declaration::Declaration::GlobalDecl(a) => {
                            let mut var_name = String::from("");
                            if let l_value::LValue::TypePair(tp, _) = a.lvalues.first().unwrap() {
                                var_name.push_str(&tp.clone().0);
                                if let VarType::Record(r) = &tp.1 {
                                    if cu.records.contains_key(&r.id) {
                                        continue;
                                    }
                                    let mut field_map = HashMap::new();
                                    for i in 0..r.fields.len() {
                                        let field_name = r.fields[i].clone().0;
                                        field_map.insert(field_name, (i * 8) as u64);
                                    }
                                    cu.records.insert(tp.clone().0, field_map);
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    for decl in p.decls {
        match decl {
            declaration::Declaration::Function(f) => {
                let body = ir_types::trans_function_defn(f.clone(), &mut cu, opt_options.cf)?;
                let fn_decl = ir_types::HIRFuncDecl {
                    body: Some(body),
                    ..cu.functions.get(&f.identifier).unwrap().clone()
                };
                cu.functions.insert(f.identifier, fn_decl);
            }
            _ => (),
        };
    }

    //println!("{:?}", cu);
    Ok(cu)
}

pub fn lower_ir(hircu: ir_types::HIRCompUnit) -> ir_types::LIRCompUnit {
    ir_types::LIRCompUnit {
        name: hircu.name.clone(),
        functions: hircu
            .functions
            .iter()
            .filter_map(|(s, hir_fd)| match &hir_fd.body {
                None => None,
                Some(body) => Some((
                    s.clone(),
                    ir_types::LIRFuncDecl {
                        name: hir_fd.name.clone(),
                        body: ir_types::lower_stmt(body.clone(), &hircu),
                        n_returns: hir_fd.n_returns,
                    },
                )),
            })
            .collect(),
        interface_functions: hircu.interface_functions,
        data_map: hircu.data_map,
    }
}

#[inline]
/// perform basic block reordering on function body
fn reorder_fn(s: String, f: LIRFuncDecl) -> Result<(String, LIRFuncDecl), Box<dyn Error>> {
    let name = f.name.clone();
    let n_returns = f.n_returns;
    Ok((
        s,
        LIRFuncDecl {
            name,
            body: reorder::reorder(f.body),
            n_returns,
        },
    ))
}
/// Complete IR translation of .eta file given as a string in [contents], output to [filename]
/// returns an Err if const folding yields overflows or exceptions
pub fn irgen(
    f: File,
    filename: String,
    libpath: &str,
    opt_options: &OptOptions,
) -> Result<String, Box<dyn Error>> {
    let mut p_file = String::from(&filename[..filename.len() - 3]);
    if let Some(p) = p_file.rfind("/") {
        p_file = p_file[p + 1..].to_string()
    }
    match f {
        file::File::Program(program::Program {
            uses,
            decls,
            line_col,
        }) => {
            let p = program::Program {
                uses,
                decls,
                line_col,
            };
            let hir_cu = translate_program_hir(p, p_file, libpath, opt_options)?;

            let LIRCompUnit {
                name,
                functions,
                interface_functions,
                data_map,
            } = lower_ir(hir_cu.clone());

            let reordered_lir_cu = LIRCompUnit {
                name,
                data_map,
                functions: functions
                    .into_iter()
                    .map(|(s, f)| reorder_fn(s, f))
                    .collect::<Result<HashMap<_, _>, _>>()?,
                interface_functions,
            };
            //return Ok(lower_ir(hir_cu).to_string());
            //println!("{}", lower_ir(hir_cu));
            //println!("{:?}", reordered_lir_cu);
            Ok(reordered_lir_cu.to_string())
        }
        _ => Ok("".to_string()),
    }
}

pub fn irgen_cu(
    f: File,
    filename: String,
    libpath: &str,
    opt_options: &OptOptions,
) -> Result<LIRCompUnit, Box<dyn Error>> {
    let mut p_file = String::from(&filename[..filename.len() - 3]);
    if let Some(p) = p_file.rfind("/") {
        p_file = p_file[p + 1..].to_string()
    }
    match f {
        file::File::Program(program::Program {
            uses,
            decls,
            line_col,
        }) => {
            let p = program::Program {
                uses,
                decls,
                line_col,
            };
            let hir_cu = translate_program_hir(p, p_file, libpath, opt_options)?;

            let LIRCompUnit {
                name,
                functions,
                interface_functions,
                data_map,
            } = lower_ir(hir_cu.clone());

            let reordered_lir_cu = LIRCompUnit {
                name,
                data_map,
                functions: functions
                    .into_iter()
                    .map(|(s, f)| reorder_fn(s, f))
                    .collect::<Result<HashMap<_, _>, _>>()?,
                interface_functions,
            };
            //return Ok(lower_ir(hir_cu).to_string());
            //println!("{}", lower_ir(hir_cu).to_string());
            //println!("{:?}", reordered_lir_cu);
            Ok(reordered_lir_cu)
        }
        _ => Ok(LIRCompUnit {
            name: "".to_string(),
            functions: HashMap::new(),
            interface_functions: vec![],
            data_map: HashMap::new(),
        }),
    }
}
