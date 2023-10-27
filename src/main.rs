use ansi_term::Style;
use etac_emw236::cfg;
use etac_emw236::eta_typechecker;
use etac_emw236::rho_typechecker;
use etac_emw236::translate::OptOptions;
use etac_emw236::translate_types::LIRNode;
use etac_emw236::{
    eta_irgen, eta_lexer, eta_parser, rho_irgen, rho_lexer, rho_parser, ssa, translate,
};

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use etac_emw236::load_file;

#[derive(Debug, Default, Clone)]
struct Options {
    lex: bool,
    parse: bool,
    typecheck: bool,
    irgen: bool,
    irrun: bool,
    files: Vec<String>,
    diag_dest: Option<String>,
    assembly_dest: Option<String>,
    source: Option<String>,
    libpath: Option<String>,
    cf: bool,
    target_os: Option<String>,
    print_help: bool,
    report_opts: bool,
    reg: bool,
    copy: bool,
    dce: bool,
    inl: bool,
    optir_initial: bool,
    optir_final: bool,
    optcfg_initial: bool,
    optcfg_final: bool,
}

fn unmangle_name(name: &str) -> String {
    let parts: Vec<&str> = name.split('_').collect();
    let mangled_name = parts[1]; // This is the name with the leading 'I'
    mangled_name[1..].to_string() // Skip the first character
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = std::env::args().collect();

    let mut options = Options::default();
    options.cf = true;
    options.reg = true;
    options.dce = true;
    options.copy = true;
    options.inl = true;

    let mut set_reg = false;
    let mut set_cf = false;
    let mut set_dce = false;
    let mut set_copy = false;
    let mut set_inl = false;

    let mut args_iter = args.iter();
    args_iter.next();
    'loop_args: loop {
        match args_iter.next() {
            Some(arg) => match arg.as_str() {
                "--lex" => options.lex = true,
                "--parse" => options.parse = true,
                "--typecheck" => options.typecheck = true,
                "--irgen" => options.irgen = true,
                "--irrun" => options.irrun = true,
                "--Oreg" => {
                    options.reg = true;
                    set_reg = true;
                    if !set_cf && options.cf {
                        options.cf = false;
                    }
                    if !set_dce && options.dce {
                        options.dce = false;
                    }
                    if !set_copy && options.copy {
                        options.copy = false;
                    }
                    if !set_inl && options.inl {
                        options.inl = false;
                    }
                }
                "--Ocopy" => {
                    options.copy = true;
                    set_copy = true;
                    if !set_reg && options.reg {
                        options.reg = false;
                    }
                    if !set_cf && options.cf {
                        options.cf = false;
                    }
                    if !set_dce && options.dce {
                        options.dce = false;
                    }
                    if !set_inl && options.inl {
                        options.inl = false;
                    }
                }
                "--Odce" => {
                    options.dce = true;
                    set_dce = true;
                    if !set_reg && options.reg {
                        options.reg = false;
                    }
                    if !set_copy && options.copy {
                        options.copy = false;
                    }
                    if !set_cf && options.cf {
                        options.cf = false;
                    }
                    if !set_inl && options.inl {
                        options.inl = false;
                    }
                }
                "--Ocf" => {
                    options.cf = true;
                    set_cf = true;
                    if !set_reg && options.reg {
                        options.reg = false;
                    }
                    if !set_copy && options.copy {
                        options.copy = false;
                    }
                    if !set_dce && options.dce {
                        options.dce = false;
                    }
                    if !set_inl && options.inl {
                        options.inl = false;
                    }
                }
                "--Oinl" => {
                    options.inl = true;
                    set_inl = true;
                    if !set_reg && options.reg {
                        options.reg = false;
                    }
                    if !set_copy && options.copy {
                        options.copy = false;
                    }
                    if !set_dce && options.dce {
                        options.dce = false;
                    }
                    if !set_cf && options.cf {
                        options.cf = false;
                    }
                }
                "--optir" => match args_iter.next() {
                    Some(s) => {
                        if s == "initial" {
                            options.optir_initial = true;
                        } else if s == "final" {
                            options.optir_final = true;
                        } else {
                            options.print_help = true;
                            break 'loop_args;
                        }
                    }
                    _ => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "--optcfg" => match args_iter.next() {
                    Some(s) => {
                        if s == "initial" {
                            options.optcfg_initial = true;
                        } else if s == "final" {
                            options.optcfg_final = true;
                        } else {
                            options.print_help = true;
                            break 'loop_args;
                        }
                    }
                    _ => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "-report-opts" => {
                    options.report_opts = true;
                }
                "-sourcepath" => match args_iter.next() {
                    Some(path) => options.source = Some(path.to_string()),
                    None => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "-D" => match args_iter.next() {
                    Some(path) => options.diag_dest = Some(path.to_string()),
                    None => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "-d" => match args_iter.next() {
                    Some(path) => options.assembly_dest = Some(path.to_string()),
                    None => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "-libpath" => match args_iter.next() {
                    Some(path) => options.libpath = Some(path.to_string()),
                    None => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "-target" => match args_iter.next() {
                    Some(path) => {
                        if path == "linux" {
                            options.target_os = Some(path.to_string())
                        }
                    }
                    None => {
                        options.print_help = true;
                        break 'loop_args;
                    }
                },
                "-O" => {
                    options.cf = false;
                    options.reg = false;
                    options.dce = false;
                    options.copy = false;
                    options.inl = false;
                }
                _ => {
                    if arg.ends_with(".eta")
                        || arg.ends_with(".eti")
                        || arg.ends_with(".rh")
                        || arg.ends_with(".ri")
                    {
                        options.files.push(arg.to_string())
                    } else {
                        options.print_help = true;
                        break 'loop_args;
                    }
                }
            },
            None => break,
        }
    }
    run_args(options);
}

/// Run options after parsing all commands
fn run_args(options: Options) {
    let opt_options = OptOptions {
        cf: options.cf,
        reg: options.reg,
        dce: options.dce,
        copy: options.copy,
        inl: options.inl,
    };

    let false_options = OptOptions {
        cf: false,
        reg: false,
        dce: false,
        copy: false,
        inl: false,
    };

    let source_path = convert_to_path(options.source);
    let diag_dest_path = convert_to_path(options.diag_dest);
    let ass_dest_path = convert_to_path(options.assembly_dest);
    let lib_path = convert_to_path(options.libpath);
    if (options.files.len() == 0 && !options.report_opts) || options.print_help {
        println!("Condition satisfied");
        print_help();
    }
    if options.lex {
        for filename in &options.files {
            let contents = match eta_lexer::load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" {
                let mut out_file = String::from(&filename[..filename.len() - 4]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".lexed");
                string_to_file(&out_file, &eta_lexer::lex(&contents));
            }
            if ext == "rh" {
                let mut out_file = String::from(&filename[..filename.len() - 3]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".lexed");
                string_to_file(&out_file, &rho_lexer::lex(&contents));
            }
        }
    }
    if options.parse {
        for filename in &options.files {
            let contents = match load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" || ext == "eti" {
                let mut out_file = String::from(&filename[..filename.len() - 4]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".parsed");
                if let Some(lex_error) = check_eta_lex_error(&contents) {
                    string_to_file(&out_file, &lex_error);
                    println!("{}", lex_error);
                    continue;
                }
                if ext == ".eti" {
                    string_to_file(
                        &out_file,
                        &eta_parser::parse_to_string(eta_parser::parse_interface(&contents)),
                    );
                } else {
                    string_to_file(
                        &out_file,
                        &eta_parser::parse_to_string(eta_parser::parse(&contents)),
                    );
                }
            }
            if ext == "rh" || ext == "ri" {
                let mut out_file = String::from(&filename[..filename.len() - 3]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".parsed");
                if let Some(lex_error) = check_rho_lex_error(&contents) {
                    string_to_file(&out_file, &lex_error);
                    println!("{}", lex_error);
                    continue;
                }
                if ext == ".ri" {
                    string_to_file(
                        &out_file,
                        &rho_parser::parse_to_string(rho_parser::parse_interface(&contents)),
                    );
                } else {
                    string_to_file(
                        &out_file,
                        &rho_parser::parse_to_string(rho_parser::parse(&contents)),
                    );
                }
            }
        }
    }
    if options.typecheck {
        for filename in &options.files {
            let contents = match load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" || ext == "eti" {
                let mut out_file = String::from(&filename[..filename.len() - 4]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".typed");
                if let Some(lex_error) = check_eta_lex_error(&contents) {
                    println!(
                        "Lexical error beginning at {}:{}",
                        filename,
                        lex_error.replace("error:", "")
                    );
                    continue;
                }
                string_to_file(
                    &out_file,
                    &eta_typechecker::type_check_to_string(eta_typechecker::type_check(
                        &contents, &lib_path, filename,
                    )),
                );
            }
            if ext == "rh" || ext == "ri" {
                let mut out_file = String::from(&filename[..filename.len() - 3]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".typed");
                if let Some(lex_error) = check_rho_lex_error(&contents) {
                    println!(
                        "Lexical error beginning at {}:{}",
                        filename,
                        lex_error.replace("error:", "")
                    );
                    continue;
                }
                string_to_file(
                    &out_file,
                    &rho_typechecker::type_check_to_string(rho_typechecker::type_check(
                        &contents, &lib_path, filename,
                    )),
                );
            }
        }
    }

    if options.irgen {
        for filename in &options.files {
            let contents = match load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" {
                let mut out_file = String::from(&filename[..filename.len() - 4]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".ir");
                let typecheck_out =
                    eta_typechecker::type_check_file(&contents, &lib_path, filename);
                match typecheck_out {
                    Ok(f) => match eta_irgen::irgen(f, filename.clone(), &lib_path, &opt_options) {
                        Ok(c) => string_to_file(&out_file, &c),
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => println!("{}", e),
                }
            }
            if ext == "rh" {
                let mut out_file = String::from(&filename[..filename.len() - 3]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".ir");
                let typecheck_out =
                    rho_typechecker::type_check_file(&contents, &lib_path, filename);
                match typecheck_out {
                    Ok(f) => match rho_irgen::irgen(f, filename.clone(), &lib_path, &opt_options) {
                        Ok(c) => string_to_file(&out_file, &c),
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
    if options.irrun {
        for filename in &options.files {
            let contents = match load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" {
                let mut out_file = String::from(&filename[..filename.len() - 4]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".ir");
                let typecheck_out =
                    eta_typechecker::type_check_file(&contents, &lib_path, filename);
                match typecheck_out {
                    Ok(f) => match eta_irgen::irgen(f, filename.clone(), &lib_path, &opt_options) {
                        Ok(c) => {
                            string_to_file(&out_file, &c);
                            env::set_current_dir(Path::new("ir_interpreter/")).unwrap();
                            Command::new("java")
                                .arg("-cp")
                                .arg("./build/:./lib/java_cup.jar:/lib/jflex.jar")
                                .arg("edu.cornell.cs.cs4120.xic.ir.interpret.Main")
                                .arg(c)
                                .spawn()
                                .expect("irrun failed");
                            env::set_current_dir(Path::new("../")).unwrap();
                        }
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => println!("{}", e),
                }
            }
            if ext == "rh" {
                let mut out_file = String::from(&filename[..filename.len() - 3]);
                out_file.insert_str(0, &diag_dest_path);
                out_file.push_str(".ir");
                let typecheck_out =
                    rho_typechecker::type_check_file(&contents, &lib_path, filename);
                match typecheck_out {
                    Ok(f) => match rho_irgen::irgen(f, filename.clone(), &lib_path, &opt_options) {
                        Ok(c) => {
                            string_to_file(&out_file, &c);
                            env::set_current_dir(Path::new("ir_interpreter/")).unwrap();
                            Command::new("java")
                                .arg("-cp")
                                .arg("./build/:./lib/java_cup.jar:/lib/jflex.jar")
                                .arg("edu.cornell.cs.cs4120.xic.ir.interpret.Main")
                                .arg(c)
                                .spawn()
                                .expect("irrun failed");
                            env::set_current_dir(Path::new("../")).unwrap();
                        }
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
    if options.optcfg_initial || options.optir_initial {
        for filename in &options.files {
            let contents = match load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" {
                let module_name = String::from(&filename[..filename.len() - 4]);
                let typecheck_out =
                    eta_typechecker::type_check_file(&contents, &lib_path, filename);

                match typecheck_out {
                    Ok(f) => {
                        match eta_irgen::irgen_cu(f, filename.clone(), &lib_path, &false_options) {
                            Ok(c) => {
                                if options.optir_initial {
                                    let mut out_file = module_name.clone();
                                    out_file.insert_str(0, &diag_dest_path);
                                    out_file.push_str("_initial.ir");
                                    string_to_file(&out_file, &c.to_string());
                                }
                                if options.optcfg_initial {
                                    for (_, function) in c.functions {
                                        let mut out_file = module_name.clone();
                                        out_file.push_str("_");
                                        out_file.push_str(&unmangle_name(&String::from(
                                            &function.name,
                                        )));
                                        out_file.push_str("_initial");
                                        out_file.insert_str(0, &diag_dest_path);
                                        out_file.push_str(".dot");

                                        let lirtree = function
                                            .body
                                            .into_iter()
                                            .map(|x| x.convert())
                                            .collect::<Vec<LIRNode>>();

                                        let cfg = cfg::CFG::new(&lirtree);

                                        string_to_file(&out_file, &cfg.debug_cfg_string());
                                    }
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            if ext == "rh" {
                let module_name = String::from(&filename[..filename.len() - 3]);
                let typecheck_out =
                    rho_typechecker::type_check_file(&contents, &lib_path, filename);

                match typecheck_out {
                    Ok(f) => {
                        match rho_irgen::irgen_cu(f, filename.clone(), &lib_path, &false_options) {
                            Ok(c) => {
                                if options.optir_initial {
                                    let mut out_file = module_name.clone();
                                    out_file.insert_str(0, &diag_dest_path);
                                    out_file.push_str("_initial.ir");
                                    string_to_file(&out_file, &c.to_string());
                                }
                                if options.optcfg_initial {
                                    for (_, function) in c.functions {
                                        let mut out_file = module_name.clone();
                                        out_file.push_str("_");
                                        out_file.push_str(&unmangle_name(&String::from(
                                            &function.name,
                                        )));
                                        out_file.push_str("_initial");
                                        out_file.insert_str(0, &diag_dest_path);
                                        out_file.push_str(".dot");

                                        let lirtree = function
                                            .body
                                            .into_iter()
                                            .map(|x| x.convert())
                                            .collect::<Vec<LIRNode>>();

                                        let cfg = cfg::CFG::new(&lirtree);

                                        string_to_file(&out_file, &cfg.debug_cfg_string());
                                    }
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
    }

    if options.optcfg_final || options.optir_final {
        for filename in &options.files {
            let contents = match load_file(&(source_path.clone() + &filename)) {
                Ok(contents) => contents,
                Err(_e) => panic!("Problem loading file: {:?}", filename),
            };
            let ext = filename.split('.').last().unwrap();
            if ext == "eta" {
                let module_name = String::from(&filename[..filename.len() - 4]);
                let typecheck_out =
                    eta_typechecker::type_check_file(&contents, &lib_path, filename);

                match typecheck_out {
                    Ok(f) => {
                        match eta_irgen::irgen_cu(f, filename.clone(), &lib_path, &opt_options) {
                            Ok(c) => {
                                if options.optir_final {
                                    let mut out_file = module_name.clone();
                                    out_file.insert_str(0, &diag_dest_path);
                                    out_file.push_str("_initial.ir");
                                    string_to_file(&out_file, &c.to_string());
                                }
                                if options.optcfg_final {
                                    for (_, function) in c.functions {
                                        let mut out_file = module_name.clone();
                                        out_file.push_str("_");
                                        out_file.push_str(&unmangle_name(&String::from(
                                            &function.name,
                                        )));
                                        out_file.push_str("_final");
                                        out_file.insert_str(0, &diag_dest_path);
                                        out_file.push_str(".dot");

                                        let lirtree = function
                                            .body
                                            .into_iter()
                                            .map(|x| x.convert())
                                            .collect::<Vec<LIRNode>>();

                                        let cfg = cfg::CFG::new(&lirtree);

                                        string_to_file(&out_file, &cfg.debug_cfg_string());
                                    }
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            if ext == "rh" {
                let module_name = String::from(&filename[..filename.len() - 3]);
                let typecheck_out =
                    rho_typechecker::type_check_file(&contents, &lib_path, filename);

                match typecheck_out {
                    Ok(f) => {
                        match rho_irgen::irgen_cu(f, filename.clone(), &lib_path, &opt_options) {
                            Ok(c) => {
                                if options.optir_final {
                                    let mut out_file = module_name.clone();
                                    out_file.insert_str(0, &diag_dest_path);
                                    out_file.push_str("_final.ir");
                                    string_to_file(&out_file, &c.to_string());
                                }
                                if options.optcfg_final {
                                    for (_, function) in c.functions {
                                        let mut out_file = module_name.clone();
                                        out_file.push_str("_");
                                        out_file.push_str(&unmangle_name(&String::from(
                                            &function.name,
                                        )));
                                        out_file.push_str("_final");
                                        out_file.insert_str(0, &diag_dest_path);
                                        out_file.push_str(".dot");

                                        let lirtree = function
                                            .body
                                            .into_iter()
                                            .map(|x| x.convert())
                                            .collect::<Vec<LIRNode>>();

                                        let cfg = cfg::CFG::new(&lirtree);

                                        string_to_file(&out_file, &cfg.debug_cfg_string());
                                    }
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
    }

    if options.report_opts {
        println!("reg");
        println!("copy");
        println!("dce");
        println!("cf");
        println!("inl");
    }

    for filename in &options.files {
        let contents = match load_file(&(source_path.clone() + &filename)) {
            Ok(contents) => contents,
            Err(_e) => panic!("Problem loading file: {:?}", filename),
        };
        let ext = filename.split('.').last().unwrap();
        if ext == "eta" {
            if let Some(lex_error) = check_eta_lex_error(&contents) {
                println!("{}", lex_error);
                continue;
            }
            let typecheck_out = eta_typechecker::type_check_file(&contents, &lib_path, filename);
            match typecheck_out {
                Ok(f) => match eta_irgen::irgen_cu(f, filename.clone(), &lib_path, &opt_options) {
                    Ok(c) => {
                        let module_name = String::from(&filename[..filename.len() - 4]);
                        //let s = ssa::convert_to_ssa(&c);
                        let ass_str = translate::translate_cu(c, &opt_options);
                        let mut out_file = module_name;
                        out_file.insert_str(0, &ass_dest_path);
                        out_file.push_str(".s");
                        string_to_file(&out_file, &ass_str);
                    }
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            }
        }
        if ext == "rh" {
            if let Some(lex_error) = check_rho_lex_error(&contents) {
                println!("{}", lex_error);
                continue;
            }
            let typecheck_out = rho_typechecker::type_check_file(&contents, &lib_path, filename);
            match typecheck_out {
                Ok(f) => match rho_irgen::irgen_cu(f, filename.clone(), &lib_path, &opt_options) {
                    Ok(c) => {
                        let module_name = String::from(&filename[..filename.len() - 3]);
                        //let s = ssa::convert_to_ssa(&c);
                        let ass_str = translate::translate_cu(c, &opt_options);
                        let mut out_file = module_name;
                        out_file.insert_str(0, &ass_dest_path);
                        out_file.push_str(".s");
                        string_to_file(&out_file, &ass_str);
                    }
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn convert_to_path(option: Option<String>) -> String {
    let mut path = option.unwrap_or_default();
    if path != "" {
        path.push_str("/");
    }
    path
}

/// Use the lex function to check for lexing errors
fn check_eta_lex_error(contents: &str) -> Option<String> {
    let lex_output = eta_lexer::lex(contents);
    let last = lex_output.lines().last();
    match last {
        Some(l) => l
            .to_string()
            .contains("error:Failure to lex")
            .then(|| l.to_string()),
        None => None,
    }
}

fn check_rho_lex_error(contents: &str) -> Option<String> {
    let lex_output = rho_lexer::lex(contents);
    let last = lex_output.lines().last();
    match last {
        Some(l) => l
            .to_string()
            .contains("error:Failure to lex")
            .then(|| l.to_string()),
        None => None,
    }
}

/// Outputs String output from processing into the designated file specified
fn string_to_file(filepath: &str, contents: &str) {
    let mut f = File::create(filepath).unwrap();
    write!(&mut f, "{}", contents).unwrap();
}

/// Prints help information for running the "etac" program to the console
fn print_help() {
    println!(
        "{} {} [OPTIONS] <SOURCE FILES>\n",
        Style::new().bold().underline().paint("Usage:"),
        Style::new().bold().paint("etac")
    );
    println!(
        "{} \n  <SOURCE FILES>  Source files to perform analysis on (can specify multiple)\n",
        Style::new().bold().underline().paint("Arguments:"),
    );
    println!(
        "{} \n  {:<width$}{:<width$}\n  {:<width$}{:<width$}\n  {:<width$}{:<width$}\n  {:<width$}{:<width$}\n  {:<width$}{:<width$}\n  {:<width$}{:<width$}\n {:<width$}{:<width$}\n {:<width$}{:<width$}\n {:<width$}{:<width$}\n {:<width$}{:<width$}\n {:<width$}{:<width$}\n {:<width$}{:<width$}\n {:<width$}{:<width$}" ,
        Style::new().bold().underline().paint("Options:"),
        "--help",
        "Print help",
        "--lex",
        "Generate output from lexical analysis",
        "--parse",
        "Generate output from syntactic analysis",
        "--typecheck",
        "Generate output from semantic analysis",
        "--irgen",
        "Generate intermediate code",
        "--irrun",
        "Generate and interpret intermediate code",
        "-D",
        "Place generated diagnostic files in directory relative to path",
        "-d",
        "Place generated assembly output files in directory relative to path",
        "-sourcepath",
        "Specify directory where to find input source files",
        "-libpath",
        "Specify directory where to find library interface files",
        "-O",
        "Disable Optimizations",
        "-target",
        "Specify the operating system for which to generate code",
        "-report-opts",
        "Print out the optimizations that are supported by this compiler",
        width = 15
    );
}
