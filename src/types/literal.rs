/// A literal value in Eta
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    String(String),
    Char(char),
    Null,
}
