use super::interface::Interface;
use super::program::Program;

/// File represents the type of a file in Eta
#[derive(Debug)]
pub enum File {
    Program(Program),
    Interface(Interface),
}
