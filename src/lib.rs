use std::fs;

#[macro_use]
extern crate pest_derive;
extern crate pest;
pub mod analysis;
pub mod cfg;
pub mod copyprop;
pub mod cost;
pub mod dce;
pub mod eta_irgen;
pub mod eta_lexer;
pub mod eta_parser;
pub mod eta_typechecker;
pub mod function_analysis;
pub mod instructions;
pub mod ir_types;
pub mod loop_opt;
pub mod regalloc;
pub mod reorder;
pub mod rho_irgen;
pub mod rho_lexer;
pub mod rho_parser;
pub mod rho_typechecker;
pub mod ssa;
pub mod tiles;
pub mod translate;
pub mod translate_types;
pub mod types;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod eta {
    #[derive(Parser)]
    #[grammar = "eta.pest"]
    pub struct EtaParser;
}
pub mod rho {
    #[derive(Parser)]
    #[grammar = "rho.pest"]
    pub struct RhoParser;
}

pub fn load_file(filepath: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filepath)
}

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn next_int() -> usize {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    CALL_COUNT.load(Ordering::SeqCst)
}
fn next_signed() -> isize {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    CALL_COUNT.load(Ordering::SeqCst) as isize
}
fn get_next_temp(tag: &str) -> String {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    format!("_{tag}{}", CALL_COUNT.load(Ordering::SeqCst))
}
fn reset() {
    CALL_COUNT.fetch_sub(CALL_COUNT.load(Ordering::SeqCst), Ordering::SeqCst);
}
