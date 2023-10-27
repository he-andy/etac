use std::cell::RefCell;

use crate::eta::{EtaParser, Rule};
use crate::eta_lexer;
use crate::types::{
    array::Array,
    array_index::ArrayIndex,
    array_literal::ArrayLiteral,
    assignment::Assignment,
    declaration::{Declaration, Use},
    expr::{Base, Bop, Expr, Uop},
    file::File,
    function::Function,
    function_call::FunctionCall,
    if_stmt::If,
    interface::Interface,
    interface_function::InterfaceFn,
    l_value::LValue,
    literal::Literal,
    primary::Primary,
    procedure_call::ProcedureCall,
    program::Program,
    statement::Statement,
    var_type::VarType,
    while_stmt::While,
    AstNode,
};
use pest::error::Error;
use pest::error::LineColLocation::Pos;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;
use pest::Parser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use crate::pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(op_or, Left))
            .op(Op::infix(op_and, Left))
            .op(Op::infix(op_eq, Left) | Op::infix(op_neq, Left))
            .op(Op::infix(op_lt, Left) | Op::infix(op_leq, Left) | Op::infix(op_geq, Left) | Op::infix(op_gt, Left))
            .op(Op::infix(op_plus, Left) | Op::infix(op_minus, Left)).op(Op::infix(op_multiply, Left) | Op::infix(op_high_multiply, Left) | Op::infix(op_divide, Left) | Op::infix(op_modulo, Left))
            .op(Op::prefix(op_negation) | Op::prefix(op_not))
    };
}

/// precondition: input is pair with rule array_rv or array_lv
/// Function to parse a pair into an ArrayIndex object
fn parse_array_index(pair: Pair<Rule>) -> ArrayIndex {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap();
    let base = Box::new(match name.as_rule() {
        Rule::identifier => {
            Base::Identifier(name.as_str().to_string(), RefCell::new(None), line_col)
        }
        Rule::expr => Base::Expr(parse_expr(name.into_inner())),
        Rule::array_literal => Base::ArrayLiteral(parse_array_literal(name)),
        Rule::method_call => Base::FunctionCall(parse_function_call(name)),
        Rule::string => Base::StringLiteral(eta_lexer::parse_string(name.as_str())),
        _ => unreachable!("Array base should not be {:?}", name),
    });

    inner
        .next()
        .unwrap()
        .into_inner()
        .fold(ArrayIndex::Base(base), |acc, x| {
            ArrayIndex::Index(
                Box::new(acc),
                Box::new(parse_expr(x.into_inner().next().unwrap().into_inner())),
                line_col.0,
                line_col.1,
            )
        })
}

///precondition: input is pairs that is an list of expr
/// Function to parse a list of expressions into a Vector of Expressions
fn parse_expr_list(list: Pairs<'_, Rule>) -> Vec<Expr> {
    list.map(|expr| parse_expr(expr.into_inner())).collect()
}

/// Function to parse a function application into a FunctionCall object
fn parse_function_call(pair: Pair<Rule>) -> FunctionCall {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    FunctionCall {
        identifier: inner.next().unwrap().as_str().to_string(),
        args: inner
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| Box::new(parse_expr(pair.into_inner())))
            .collect(),
        rc: RefCell::new(None),
        line_col,
    }
}

/// Function to parse a length expression into a FunctionCall object
fn parse_length_expr(pair: Pair<Rule>) -> FunctionCall {
    let line_col = pair.line_col();
    FunctionCall {
        identifier: "length".to_string(),
        args: vec![Box::new(parse_expr(
            pair.into_inner().next().unwrap().into_inner(),
        ))],
        rc: RefCell::new(None),
        line_col,
    }
}
///precondition: input is pair with rule array_literal
/// Function to parse an array literal into an ArrayLiteral object
fn parse_array_literal(primary: Pair<Rule>) -> ArrayLiteral {
    let line_col = primary.line_col();
    ArrayLiteral {
        inner: primary
            .into_inner()
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| Box::new(parse_expr(pair.into_inner().next().unwrap().into_inner())))
            .collect(),
        line_col,
    }
}

///precondition: input is pairs with one element of rule inner
/// Function to parse an expression into an Expr object using a PrattParser
pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    let line_col = pairs.clone().next().unwrap().line_col();
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Primary(Primary::Literal(Literal::Int(
                primary.as_str().parse::<i64>().unwrap(),
            ))),
            Rule::bool => Expr::Primary(Primary::Literal(Literal::Bool(
                primary.as_str().parse::<bool>().unwrap(),
            ))),
            Rule::string => Expr::Primary(Primary::Literal(Literal::String(
                eta_lexer::parse_string(primary.as_str()), //parse string
            ))),
            Rule::identifier => Expr::Primary(Primary::Identifier(
                primary.as_str().to_string(),
                RefCell::new(None),
                line_col,
            )),
            Rule::char => Expr::Primary(Primary::Literal(Literal::Char(
                eta_lexer::parse_string(primary.as_str())
                    .chars()
                    .next()
                    .unwrap(), //parse string
            ))),
            Rule::array_rv => Expr::Primary(Primary::ArrayIndex(parse_array_index(primary))),
            Rule::array_literal => {
                Expr::Primary(Primary::ArrayLiteral(parse_array_literal(primary)))
            }
            Rule::expr_inner => parse_expr(primary.into_inner()),
            Rule::method_call => Expr::Primary(Primary::FunctionCall(parse_function_call(primary))),
            Rule::length_expr => Expr::Primary(Primary::FunctionCall(parse_length_expr(primary))),
            rule => unreachable!("Expr::parse expected primary, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op2 = match op.as_rule() {
                Rule::op_plus => Bop::Add,
                Rule::op_minus => Bop::Sub,
                Rule::op_high_multiply => Bop::HMult,
                Rule::op_multiply => Bop::Mult,
                Rule::op_divide => Bop::Div,
                Rule::op_modulo => Bop::Rem,
                Rule::op_and => Bop::And,
                Rule::op_or => Bop::Or,
                Rule::op_geq => Bop::Geq,
                Rule::op_gt => Bop::Gt,
                Rule::op_leq => Bop::Leq,
                Rule::op_lt => Bop::Lt,
                Rule::op_eq => Bop::Eq,
                Rule::op_neq => Bop::Neq,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };

            Expr::Bop {
                op: op2,
                left: Box::new(lhs),
                right: Box::new(rhs),
                line_col: op.line_col(),
                is_array: RefCell::new(false),
            }
        })
        .map_prefix(|op, rhs| {
            let op2 = match op.as_rule() {
                Rule::op_negation => Uop::Neg,
                Rule::op_not => Uop::Not,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            };
            Expr::Uop {
                op: op2,
                expr: Box::new(rhs),
                line_col: op.line_col(),
            }
        })
        .parse(pairs)
}

///precondition: input is pair with rule type or equivalent
/// Function to convert a Pest Pair into a VarType object
fn type_to_enum(pair: Pair<Rule>) -> VarType {
    let t = pair.into_inner().next().unwrap();
    match t.as_rule() {
        Rule::primitive_type => match t.as_str() {
            "int" => VarType::IntType,
            "bool" => VarType::BoolType,
            _ => panic!("Expected string \"int\" or \"bool\""),
        },
        Rule::array => parse_array(t),
        _ => panic!("Expected string \"int\" or \"bool\""),
    }
}

///precondition: input is pair with rule array
/// Function to parse a pair into an Array
fn parse_array(pair: Pair<Rule>) -> VarType {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    let arr_type = match inner.next().unwrap().as_str() {
        "int" => VarType::IntType,
        "bool" => VarType::BoolType,
        _ => unreachable!("Expected string \"int\" or \"bool\""),
    };
    inner.rev().fold(arr_type, |acc, x| {
        let size = match x.as_rule() {
            Rule::empty_bracket_capture => None,
            Rule::sized_bracket_capture => {
                Some(parse_expr(x.into_inner().next().unwrap().into_inner()))
            }
            _ => unreachable!("Cannot reach this case if matched on Rule::array"),
        };
        let contents = Box::new(acc);
        VarType::Array(Array {
            size,
            contents,
            line_col,
        })
    })
}

///precondition: input is pair with rule decl_unit
/// Function to convert a declaration into a TypePair
fn decl_to_type_pair(pair: Pair<Rule>) -> (String, VarType) {
    let mut inner = pair.into_inner();
    (
        String::from(inner.next().unwrap().as_str()),
        type_to_enum(inner.next().unwrap()),
    )
}

///precondition: input is pair with inner list of decl_unit
/// Function to parse function arguments into a vector of TypePairs
fn parse_args(pair: Pair<Rule>) -> Vec<(String, VarType)> {
    pair.into_inner()
        .map(|arg| decl_to_type_pair(arg))
        .collect()
}

/// precondition: input is pair with rule lvalues or decl_stmt
/// Function to parse a pair into a vector of LValues
fn parse_lvalues(pair: Pair<Rule>) -> Vec<LValue> {
    let line_col = pair.line_col();
    pair.into_inner()
        .map(|lvalue| match lvalue.as_rule() {
            Rule::underscore => LValue::Discard,
            Rule::decl_unit | Rule::xsa_type_decl_unit => {
                LValue::TypePair(decl_to_type_pair(lvalue), line_col)
            }
            Rule::identifier => {
                LValue::Identifier(lvalue.as_str().to_string(), RefCell::new(None), line_col)
            }
            Rule::array_lv => LValue::ArrayIndex(parse_array_index(lvalue), line_col),
            _ => unreachable!("Failed parsing lvalue {:?}", lvalue),
        })
        .collect()
}

/// Function to parse a return statement
fn parse_returns(pair: Pair<Rule>) -> Vec<VarType> {
    pair.into_inner()
        .map(|typestr| type_to_enum(typestr))
        .collect()
}

/// Function to parse a While block
fn parse_while(pair: Pair<Rule>) -> While {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    While {
        condition: parse_expr(inner.next().unwrap().into_inner()),
        stmt: Box::new(parse_statement(inner.next().unwrap())),
        line_col,
    }
}

///precondition: input is pair with rule if_stmt
/// Function to parse an if block
fn parse_if(pair: Pair<Rule>) -> If {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    let condition = inner.next().unwrap();
    let branch1 = inner.next().unwrap();
    let branch2 = inner.next();
    match branch2 {
        None => If {
            condition: parse_expr(condition.into_inner()),
            stmt: Box::new(parse_statement(branch1)),
            el: None,
            line_col,
        },
        Some(branch2) => If {
            condition: parse_expr(condition.into_inner()),
            stmt: Box::new(parse_statement(branch1)),
            el: Some(Box::new(parse_statement(branch2))),
            line_col,
        },
    }
}

///precondition: pair with rule globdecl
/// Function to parse a global declaration
fn parse_glob_decl(pair: Pair<Rule>) -> Assignment {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    let lvalue = inner.next().unwrap();
    let rvalue = inner.next();

    match rvalue {
        Some(rvalue) => Assignment {
            lvalues: vec![LValue::TypePair(decl_to_type_pair(lvalue), line_col)],
            rvalues: Some(vec![parse_expr(rvalue.into_inner())]),
            line_col,
        },
        None => Assignment {
            lvalues: vec![LValue::TypePair(decl_to_type_pair(lvalue), line_col)],
            rvalues: None,
            line_col,
        },
    }
}

///precondition: input is pair with rule assignment
/// Function to parse an assignment
fn parse_assignment(pair: Pair<Rule>) -> Assignment {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    let lvalues = inner.next().unwrap();
    match inner.next() {
        Some(rvalues) => Assignment {
            lvalues: parse_lvalues(lvalues),
            rvalues: Some(parse_expr_list(rvalues.into_inner())),
            line_col,
        },
        None => Assignment {
            lvalues: parse_lvalues(lvalues),
            rvalues: None,
            line_col,
        },
    }
}

///precondition: input is pair with rule method_call
/// Function to parse a procedure call
fn parse_procedure_call(pair: Pair<Rule>) -> ProcedureCall {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    ProcedureCall {
        identifier: inner.next().unwrap().as_str().to_string(),
        args: parse_expr_list(inner.next().unwrap().into_inner()),
        rc: RefCell::new(None),
        line_col,
    }
}

///precondition: takes in pair with rule that represents a valid statement type
/// Function to parse a statement
fn parse_statement(pair: Pair<Rule>) -> Statement {
    let line_col = pair.line_col();
    let inner = pair.clone().into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::stmt => match inner.as_rule() {
            Rule::block => Statement::Block(parse_block(inner)),
            Rule::while_block => Statement::While(parse_while(inner)),
            Rule::if_stmt => Statement::If(parse_if(inner)),
            Rule::assignment => Statement::Assignment(parse_assignment(inner)),
            Rule::decl_stmt => Statement::Assignment(Assignment {
                lvalues: parse_lvalues(inner),
                rvalues: None,
                line_col,
            }),
            Rule::method_call => Statement::ProcedureCall(parse_procedure_call(inner)),
            _ => unreachable!("Failed parsing command"),
        },
        Rule::return_stmt_wrapper => Statement::Return(
            inner
                .into_inner()
                .map(|expr| parse_expr(expr.into_inner()))
                .collect(),
        ),
        _ => unreachable!("Failed parsing statement"),
    }
}

///precondition: input is pair with rule block
/// Function to parse a block of statements
fn parse_block(pair: Pair<Rule>) -> Vec<Box<Statement>> {
    pair.into_inner()
        .map(|statement| Box::new(parse_statement(statement)))
        .collect()
}

///precondition: input is pair with rule function
/// Function to parse a function or procedure
fn parse_function(pair: Pair<Rule>) -> Function {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap();
    let args = inner.next().unwrap();
    let item3 = inner.next().unwrap();
    let item4 = inner.next();

    match item4 {
        Some(block) => {
            let identifier = String::from(id.as_str());
            let args = parse_args(args);
            let returns = parse_returns(item3);
            let block = Statement::Block(parse_block(block));
            Function {
                identifier,
                args,
                returns: Some(returns),
                block,
                line_col,
            }
        }
        None => {
            let identifier = String::from(id.as_str());
            let args = parse_args(args);
            let block = Statement::Block(parse_block(item3));
            Function {
                identifier,
                args,
                returns: None,
                block,
                line_col,
            }
        }
    }
}

/// Function to parse a function or procedure declaration in .eti files
pub fn parse_interface_fn(pair: Pair<Rule>) -> InterfaceFn {
    let line_col = pair.line_col();
    let mut inner = pair.into_inner().into_iter();
    let id = inner.next().unwrap();
    let args = inner.next();
    let returns = inner.next();

    match returns {
        Some(returns) => {
            let identifier = String::from(id.as_str());
            let args = match args {
                Some(args) => parse_args(args),
                None => vec![],
            };
            let returns = parse_returns(returns);
            InterfaceFn {
                identifier,
                args,
                returns: Some(returns),
                line_col,
            }
        }
        None => {
            let identifier = String::from(id.as_str());
            let args = match args {
                Some(args) => parse_args(args),
                None => vec![],
            };
            InterfaceFn {
                identifier,
                args,
                returns: None,
                line_col,
            }
        }
    }
}

/// Function to parse an Eta program
pub fn parse(contents: &str) -> Result<File, Error<Rule>> {
    let pest_program = EtaParser::parse(Rule::program, contents)?.next().unwrap();
    let line_col = pest_program.line_col();
    let mut program = pest_program.into_inner();
    let uses = program
        .next()
        .unwrap()
        .into_inner()
        .map(|x| Use {
            identifier: x.into_inner().next().unwrap().as_str().to_string(),
        })
        .collect();

    let decls = program
        .next()
        .unwrap()
        .into_inner()
        .map(|x| match x.as_rule() {
            Rule::function => Declaration::Function(parse_function(x)),
            Rule::globdecl => Declaration::GlobalDecl(parse_glob_decl(x)),
            _ => unreachable!("This case should never occur."),
        })
        .collect();

    let out = File::Program(Program {
        uses,
        decls,
        line_col,
    });
    //println!("{:?}", out);
    Ok(out)
}

/// Function to parse an Eta interface file
pub fn parse_interface(contents: &str) -> Result<File, Error<Rule>> {
    let pest_program = EtaParser::parse(Rule::interface, contents)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();

    let line_col = pest_program.line_col();
    let program = pest_program.into_inner();

    let decls = program
        .map(|x| Declaration::InterfaceFn(parse_interface_fn(x)))
        .collect();

    let out = File::Interface(Interface {
        uses: vec![],
        decls,
        line_col,
    });

    //println!("{}", out.to_string());
    Ok(out)
}

/// Function to output results of parsing to a string
pub fn parse_to_string(prog: Result<File, Error<Rule>>) -> String {
    match prog {
        Ok(p) => match p {
            File::Interface(i) => i.to_string(),
            File::Program(p) => p.to_string(),
        },
        Err(e) => {
            if let Pos((x, y)) = e.line_col {
                let s = format!(
                    "{}:{} error:{}",
                    x,
                    y,
                    &e.to_string().lines().last().unwrap()[4..]
                );
                // println!("{s}");
                s
            } else {
                unreachable!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::load_file;

    use super::*;

    fn file_to_str(filepath: &str) -> String {
        let contents = match load_file(&filepath) {
            Ok(contents) => contents,
            Err(_e) => panic!("Problem loading file: {:?}", filepath),
        };
        return contents;
    }

    // whitespace
    fn remove_whites(s: String) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn test_global_decl() {
        let input = "x:int = 5;";
        let expected = "(()((:globalxint5)))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_global_negative_literal() {
        let input = "x:int = -2";
        let expected = "(()((:globalxint-2)))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_global_decl_complex() {
        let input = "a:int[2+3]";
        let expected = "1:9 error: unexpected integer";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_global_decl_composed() {
        let input = file_to_str("tests/pa2_tests/global.eta");
        let expected = file_to_str("tests/pa2_tests/global.parsed");
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_global_error() {
        let input = file_to_str("tests/pa2_tests/global_error.eta");
        let expected = file_to_str("tests/pa2_tests/global_error.parsed");
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_index_string_literal() {
        let input = "bar() { a:int = \"hello\"[0] }";
        let expected = "(()((bar()()((=(aint)([]hello0))))))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_excessive_semicolons() {
        let input = "x:int = 5;;";
        let expected = "1:11 error: expectedEOI, identifier, or globdecl";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_multi_decl() {
        let input = "bar() { x: bool, y: int = true, 3 }";
        let expected = "(()((bar()()((=((xbool)(yint))true3)))))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_illegal_array_decl1() {
        let input = "bar() { err1: int[3] = {1,2,3} }";
        let expected =
            "1:22error:expected empty_bracket_capture, sized_bracket_capture, return_stmt, or stmt";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_illegal_array_decl2() {
        let input = "bar() { err1: int[][3] }";
        let expected = "1:20 error: expected empty_bracket_capture, return_stmt, orstmt";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_empty_function() {
        let input = "bar() { }";
        let expected = "(()((bar()()())))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_wrong_type_operation1() {
        let input = "bar() { x:int = true + 3 }";
        let expected = "(()((bar()()((=(xint)(+true3))))))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_wrong_type_operation2() {
        let input = "bar() { x:int = true + {1,2} }";
        let expected = "(()((bar()()((=(xint)(+true(12)))))))";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_invalid_if_return() {
        let input = file_to_str("tests/pa2_tests/if_invalid_return.eta");
        let expected = "3:5 error:expected stmt";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_interface_file() {
        let input = file_to_str("tests/pa2_tests/compilers.eti");
        let expected = file_to_str("tests/pa2_tests/compilers.parsed");
        assert_eq!(
            remove_whites(parse_to_string(parse_interface(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_interface_fail() {
        let input = file_to_str("tests/pa2_tests/compilers.eti");
        let expected = "5:1error:expectedreturnsorblock";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_interface_fail2() {
        let input = file_to_str("tests/pa2_tests/compilers2.eti");
        let expected = file_to_str("tests/pa2_tests/compilers2.parsed");
        assert_eq!(
            remove_whites(parse_to_string(parse_interface(&input))),
            remove_whites(expected.to_string())
        );
    }

    #[test]
    fn test_interface_correct_wrong_parser() {
        let input = file_to_str("tests/pa2_tests/compilers2.eti");
        let expected = "3:1error:expectedempty_bracket_capture,sized_bracket_capture,orblock";
        assert_eq!(
            remove_whites(parse_to_string(parse(&input))),
            remove_whites(expected.to_string())
        );
    }

    // Tests from eth Test Harness
    #[test]
    fn test_add_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/add.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/add.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_arrayinit_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/arrayinit.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/arrayinit.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_arrayinit2_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/arrayinit2.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/arrayinit2.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_beauty_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/beauty.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/beauty.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_ex1_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/ex1.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/ex1.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_ex2_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/ex2.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/ex2.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_ex3_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/ex3.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/ex3.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_gcd_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/gcd.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/gcd.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_insertionsort_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/insertionsort.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/insertionsort.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }

    #[test]
    fn test_io_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/io.eti");
        let expected = remove_whites(file_to_str("tests/syntactic_analysis_harness/io.parsedsol"));
        assert_eq!(
            remove_whites(parse_to_string(parse_interface(&input))),
            expected
        );
    }

    #[test]
    fn test_mdarrays_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/mdarrays.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/mdarrays.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_ratadd_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/ratadd.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/ratadd.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_ratadduse_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/ratadduse.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/ratadduse.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_spec1_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/spec1.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/spec1.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_spec2_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/spec2.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/spec2.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn test_spec3_harness() {
        let input = file_to_str("tests/syntactic_analysis_harness/spec3.eta");
        let expected = remove_whites(file_to_str(
            "tests/syntactic_analysis_harness/spec3.parsedsol",
        ));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }

    // Failed test cases from PA2

    #[test]
    fn failed_test1() {
        let input = file_to_str("tests/pa2_tests/assign01.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/assign01.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }

    #[test]
    fn failed_test2() {
        let input = file_to_str("tests/pa2_tests/assign02.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/assign02.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }

    #[test]
    fn failed_test3() {
        let input = file_to_str("tests/pa2_tests/block01.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/block01.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }

    #[test]
    fn failed_test4() {
        let input = file_to_str("tests/pa2_tests/block02.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/block02.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }

    #[test]
    fn failed_test5() {
        let input = file_to_str("tests/pa2_tests/expr07.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/expr07.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test6() {
        let input = file_to_str("tests/pa2_tests/if02.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/if02.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test7() {
        let input = file_to_str("tests/pa2_tests/prec06.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/prec06.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test8() {
        let input = file_to_str("tests/pa2_tests/return02.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/return02.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test9() {
        let input = file_to_str("tests/pa2_tests/use02.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/use02.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test10() {
        let input = file_to_str("tests/pa2_tests/use03.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/use03.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test11() {
        let input = file_to_str("tests/pa2_tests/vardecl05.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/vardecl05.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test12() {
        let input = file_to_str("tests/pa2_tests/use01.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/use01.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test13() {
        let input = file_to_str("tests/pa2_tests/vardecl07.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/vardecl07.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test14() {
        let input = file_to_str("tests/pa2_tests/vardecl01.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/vardecl01.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
    #[test]
    fn failed_test15() {
        let input = file_to_str("tests/pa2_tests/empty.eta");
        let expected = remove_whites(file_to_str("tests/pa2_tests/empty.parsedsol"));
        assert_eq!(remove_whites(parse_to_string(parse(&input))), expected);
    }
}
