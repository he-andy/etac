extern crate pest;
use pest::error::LineColLocation::Pos;
use pest::iterators::Pairs;
use pest::Parser;
use regex::Captures;
use regex::Regex;
use std::fmt::Write;
use std::fs;
use std::str;

use crate::rho::RhoParser as EtaLex;
use crate::rho::Rule;

/// Load a file into a string.
pub fn load_file(filepath: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filepath)
}

/// Perform lexical analysis on a string using the Pest grammar for the Eta Programming Language.
///
/// # Examples
/// ```
/// let input = "x:int = 2;";
/// let output = etac_emw236::rho_lexer::lex(input);
/// let expected = "1:1 id x\n1:2 :\n1:3 int\n1:7 =\n1:9 integer 2\n1:10 ;\n";
///
/// assert_eq!(output, expected);
/// ```
pub fn lex(contents: &str) -> String {
    let mut output = String::from("");
    let pairs = EtaLex::parse(Rule::lexer, contents);
    match pairs {
        Ok(lexemes) => {
            out_lex(lexemes, &mut output);
        }
        Err(e) => {
            let valid_pairs = EtaLex::parse(Rule::error_lexer, contents).unwrap();
            out_lex(valid_pairs, &mut output);
            if let Pos((x, y)) = e.line_col {
                writeln!(output, "{}:{} error:Failure to lex", x, y).unwrap()
            }
        }
    }
    output
}

pub fn parse_string(str: &str) -> String {
    let re = Regex::new(r"\\x\{(?P<unicode>[0-9a-fA-F]{1,6})\}").unwrap();
    re.replace_all(&str[1..str.len() - 1], |caps: &Captures| {
        let numeric = u32::from_str_radix(&caps["unicode"], 16).unwrap();
        if 32 <= numeric && numeric <= 126 {
            std::char::from_u32(numeric).unwrap().to_string()
        } else {
            format!("\\x{{{}}}", &caps["unicode"].to_uppercase())
        }
    })
    //.replace(r"\\", r"\")
    .chars()
    .map(|c: char| {
        if c.is_ascii() {
            c.to_string()
        } else {
            format!("\\x{{{:X}}}", c as u32)
        }
    })
    .collect::<String>()
    // escape "'/
    // let re_escapes = Regex::new(r#"\\(?P<esc>[\\'"])"#).unwrap();
    // re_escapes
    //     .replace_all(&res, |caps: &Captures| caps["esc"].to_string())
    //     .to_string()
}

/// Helper function for performing lexical analysis. Pattern matches on the Pest Rule and outputs the corresponding lexical result.
fn out_lex(lexemes: Pairs<Rule>, output: &mut String) {
    for pair in lexemes {
        let (line, col) = pair.line_col();
        if let Some(token_type) = pair.clone().into_inner().next() {
            match token_type.as_rule() {
                // fix this shit evan so it aint just p.x
                Rule::identifier => {
                    writeln!(output, "{}:{} id {}", line, col, pair.as_str()).unwrap()
                }
                Rule::integer => {
                    match pair.as_str().parse::<i64>() {
                        Ok(x) => writeln!(output, "{}:{} integer {}", line, col, x).unwrap(),
                        Err(_) => {
                            writeln!(output, "{}:{} error:Failure to lex", line, col).unwrap();
                            break;
                        }
                    };
                }
                Rule::char => writeln!(
                    output,
                    "{}:{} character {}",
                    line,
                    col,
                    parse_string(pair.as_str())
                )
                .unwrap(),
                Rule::string => {
                    writeln!(
                        output,
                        "{}:{} string {}",
                        line,
                        col,
                        parse_string(pair.as_str())
                    )
                    .unwrap();
                }
                Rule::symbol
                | Rule::keyword
                | Rule::primitive_type
                | Rule::op_binary
                | Rule::op_unary
                | Rule::bool => {
                    writeln!(output, "{}:{} {}", line, col, pair.as_str()).unwrap();
                }
                _ => (),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_line_lex_correct() {
        let input = "use io";
        let expected = "1:1 use\n1:5 id io\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_one_line_lex_complex_correct() {
        let input = "print(\"Hello, Worl\\x{64}!\\n\")";
        let expected = "1:1 id print\n1:6 (\n1:7 string Hello, World!\\n\n1:29 )\n";
        assert_eq!(lex(input), expected);
    }

    // Variable Declaration Tests
    #[test]
    fn test_int_declaration_correct() {
        let input = "x:int = 2;";
        let expected = "1:1 id x\n1:2 :\n1:3 int\n1:7 =\n1:9 integer 2\n1:10 ;\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_bool_declaration_correct() {
        let input = "he_he:bool = true";
        let expected = "1:1 id he_he\n1:6 :\n1:7 bool\n1:12 =\n1:14 true\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_array_declaration_correct() {
        let input = "a: int[] = {1}";
        let expected =
            "1:1 id a\n1:2 :\n1:4 int\n1:7 [\n1:8 ]\n1:10 =\n1:12 {\n1:13 integer 1\n1:14 }\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_array2d_declaration_correct() {
        let input = "a: int[][] = {{1},{2}}";
        let expected =
        "1:1 id a\n1:2 :\n1:4 int\n1:7 [\n1:8 ]\n1:9 [\n1:10 ]\n1:12 =\n1:14 {\n1:15 {\n1:16 integer 1\n1:17 }\n1:18 ,\n1:19 {\n1:20 integer 2\n1:21 }\n1:22 }\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_string_declaration_correct() {
        let input = "s: int[] = \"Hello\" + {13, 10}";
        let expected =
        "1:1 id s\n1:2 :\n1:4 int\n1:7 [\n1:8 ]\n1:10 =\n1:12 string Hello\n1:20 +\n1:22 {\n1:23 integer 13\n1:25 ,\n1:27 integer 10\n1:29 }\n";
        assert_eq!(lex(input), expected);
    }

    // Operator Tests
    #[test]
    fn test_basic_bop_arithmetic_correct() {
        let input = "x = x + 1 -  y";
        let expected = "1:1 id x\n1:3 =\n1:5 id x\n1:7 +\n1:9 integer 1\n1:11 -\n1:14 id y\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_basic_bop_mult_correct() {
        let input = "a = b * c *>>c";
        let expected = "1:1 id a\n1:3 =\n1:5 id b\n1:7 *\n1:9 id c\n1:11 *>>\n1:14 id c\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_basic_bop_div_mod_correct() {
        let input = "hi = 13/ 5 % bye";
        let expected =
            "1:1 id hi\n1:4 =\n1:6 integer 13\n1:8 /\n1:10 integer 5\n1:12 %\n1:14 id bye\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_basic_bop_compare_correct() {
        let input = "result = (a<b) | (c > b) | (a <= b) & (a>=c)";
        let expected =
        "1:1 id result\n1:8 =\n1:10 (\n1:11 id a\n1:12 <\n1:13 id b\n1:14 )\n1:16 |\n1:18 (\n1:19 id c\n1:21 >\n1:23 id b\n1:24 )\n1:26 |\n1:28 (\n1:29 id a\n1:31 <=\n1:34 id b\n1:35 )\n1:37 &\n1:39 (\n1:40 id a\n1:41 >=\n1:43 id c\n1:44 )\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_basic_bop_equality_correct() {
        let input = "result = a== b & c != b";
        let expected =
        "1:1 id result\n1:8 =\n1:10 id a\n1:11 ==\n1:14 id b\n1:16 &\n1:18 id c\n1:20 !=\n1:23 id b\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_basic_uop_neg_correct() {
        let input = "one = -two";
        let expected = "1:1 id one\n1:5 =\n1:7 -\n1:8 id two\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_basic_uop_not_correct() {
        let input = "yes = !no";
        let expected = "1:1 id yes\n1:5 =\n1:7 !\n1:8 id no\n";
        assert_eq!(lex(input), expected);
    }

    // Keyword Tests
    #[test]
    fn test_keyword_while_return_correct() {
        let input = "while i < n { return hi, bye; }";
        let expected = "1:1 while\n1:7 id i\n1:9 <\n1:11 id n\n1:13 {\n1:15 return\n1:22 id hi\n1:24 ,\n1:26 id bye\n1:29 ;\n1:31 }\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_keyword_if_else_correct() {
        let input = "if a == b { print(\"hi\")} else {";
        let expected = "1:1 if\n1:4 id a\n1:6 ==\n1:9 id b\n1:11 {\n1:13 id print\n1:18 (\n1:19 string hi\n1:23 )\n1:24 }\n1:26 else\n1:31 {\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_keyword_length_correct() {
        let input = "a = length(b)";
        let expected = "1:1 id a\n1:3 =\n1:5 length\n1:11 (\n1:12 id b\n1:13 )\n";
        assert_eq!(lex(input), expected);
    }

    // Function Declaration Tests
    #[test]
    fn test_function_declaration_correct() {
        let input = "main(args: int[][]) {";
        let expected = "1:1 id main\n1:5 (\n1:6 id args\n1:10 :\n1:12 int\n1:15 [\n1:16 ]\n1:17 [\n1:18 ]\n1:19 )\n1:21 {\n";
        assert_eq!(lex(input), expected);
    }
    #[test]
    fn test_library_function_correct() {
        let input = "input = readln()";
        let expected = "1:1 id input\n1:7 =\n1:9 id readln\n1:15 (\n1:16 )\n";
        assert_eq!(lex(input), expected);
    }

    // Comments and Escape Tests
    #[test]
    fn test_comment_correct() {
        let input = "//this is a comment";
        let expected = "";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_escapes_correct() {
        let input = "print(\"\\'something\\\\\")";
        println!("{}\n{}", input, lex(input));
        let expected = "1:1 id print\n1:6 (\n1:7 string \\'something\\\\\n1:22 )\n";
        assert_eq!(lex(input), expected);
    }
    #[test]
    fn test_escapes2_correct() {
        let input = "print(\"\\\'something\\\\\")";
        let expected = "1:1 id print\n1:6 (\n1:7 string \\\'something\\\\\n1:22 )\n";
        assert_eq!(lex(input), expected);
    }

    // Unicode Tests
    #[test]
    fn test_emoji_correct() {
        let input = "\"laughing \\x{1f602} emoji\"";
        let expected = "1:1 string laughing \\x{1F602} emoji\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_emoji_char_correct() {
        let input = "'\\x{1f602}'";
        let expected = "1:1 character \\x{1F602}\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_emoji_flip_correct() {
        let input = "\"😂\"";
        let expected = "1:1 string \\x{1F602}\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_korean_character_correct() {
        let input = "'\\x{BBFC}'";
        let expected = "1:1 character \\x{BBFC}\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_unicode_correct() {
        let input = "\"\\x{FFFFF}\"";
        let expected = "1:1 string \\x{FFFFF}\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_unicode_symbols_correct() {
        let input = "\"\\x{25E2}\\x{30C6}\"";
        let expected = "1:1 string \\x{25E2}\\x{30C6}\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_unicode_edge_correct() {
        let input = "\"\\x{10DEAD}\"";
        let expected = "1:1 string \\x{10DEAD}\n";
        assert_eq!(lex(input), expected);
    }

    // Lexical Error Tests
    #[test]
    fn test_single_quote_error() {
        let input = "\'\\\'"; // This is '\'
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_single_quote_error2() {
        let input = "\'\'\'"; // This is '''
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_single_quote_error3() {
        let input = "\'\\\'"; // This is '\'
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_single_quote_error4() {
        let input = "\'a\'\'"; // This is 'a''
        let expected = "1:1 character a\n1:4 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_double_quote_error() {
        let input = "print(\"\"\");"; //print(""")
        let expected = "1:1 id print\n1:6 (\n1:7 string \n1:9 error:Failure to lex\n";
        println!("{}", lex(input));
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_incorrect_unicode_escape_error() {
        let input = "\"\\x{1111111}\"";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_incorrect_unicode_escape2_error() {
        let input = "\"\\x{}\"";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_incorrect_unicode_escape3_error() {
        let input = "\"\\x{FFFFFF}\"";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_incorrect_unicode_escape4_error() {
        let input = "\"\\x{110000}\"";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_incorrect_unicode_escape_char_error() {
        let input = "'\\x{FFFFFF}'";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_incorrect_escape_error() {
        let input = "\\b1111"; // Eta doesn't support this escape
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    // Tests from eth Test Harness
    #[test]
    fn test_add_harness() {
        let input = load_file("tests/lexical_analysis_harness/add.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/add.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_arrayinit_harness() {
        let input = load_file("tests/lexical_analysis_harness/arrayinit.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/arrayinit.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_arrayinit2_harness() {
        let input = load_file("tests/lexical_analysis_harness/arrayinit2.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/arrayinit2.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_beauty_harness() {
        let input = load_file("tests/lexical_analysis_harness/beauty.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/beauty.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_ex1_harness() {
        let input = load_file("tests/lexical_analysis_harness/ex1.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/ex1.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_ex2_harness() {
        let input = load_file("tests/lexical_analysis_harness/ex2.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/ex2.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_gcd_harness() {
        let input = load_file("tests/lexical_analysis_harness/gcd.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/gcd.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_insertionsort_harness() {
        let input = load_file("tests/lexical_analysis_harness/insertionsort.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/insertionsort.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_mdarrays_harness() {
        let input = load_file("tests/lexical_analysis_harness/mdarrays.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/mdarrays.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_ratadd_harness() {
        let input = load_file("tests/lexical_analysis_harness/ratadd.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/ratadd.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_ratadduse_harness() {
        let input = load_file("tests/lexical_analysis_harness/ratadduse.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/ratadduse.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_spec1_harness() {
        let input = load_file("tests/lexical_analysis_harness/spec1.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/spec1.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_spec2_harness() {
        let input = load_file("tests/lexical_analysis_harness/spec2.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/spec2.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn test_spec3_harness() {
        let input = load_file("tests/lexical_analysis_harness/spec3.eta").unwrap();
        let expected = load_file("tests/lexical_analysis_harness/spec3.lexed").unwrap();
        assert_eq!(lex(&input), expected);
    }
    // Failed test cases from PA1

    #[test]
    fn failed_test1() {
        let input = "iftrueelsefalse0";
        let expected = "1:1 id iftrueelsefalse0\n";
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn failed_test2() {
        let input = "false";
        let expected = "1:1 false\n";
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn failed_test3() {
        let input = "true";
        let expected = "1:1 true\n";
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn failed_test4() {
        let input = "'\n'";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn failed_test5() {
        let input = "\"\n\"";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(&input), expected);
    }

    #[test]
    fn failed_test6() {
        let input = "use0";
        let expected = "1:1 id use0\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn failed_test7() {
        let input = "iftrueelsefalse0";
        let expected = "1:1 id iftrueelsefalse0\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn failed_test8() {
        let input = "bool''''x'x'x'x_x2451'x";
        let expected = "1:1 id bool''''x'x'x'x_x2451'x\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn failed_test9() {
        let input = "\"This might\nnot work\"";
        let expected = "1:1 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn failed_test10() {
        let input = "b: int = 10000000000000000000000000000000";
        let expected = "1:1 id b\n1:2 :\n1:4 int\n1:8 =\n1:10 error:Failure to lex\n";
        assert_eq!(lex(input), expected);
    }
}
