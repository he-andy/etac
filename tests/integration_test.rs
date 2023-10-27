use etac_emw236::eta_typechecker;

#[test]
fn test_multiple_use() {
    let input = eta_typechecker::file_to_str("tests/pa3_tests/multiple_use.eta");
    let expected = eta_typechecker::file_to_str("tests/pa3_tests/multiple_use.typed");
    assert_eq!(
        eta_typechecker::type_check_to_string(eta_typechecker::type_check(
            &input,
            "tests/pa3_tests/",
            "multiple_use.eta"
        )),
        expected
    );
}

#[test]
fn test_enigma() {
    let input = eta_typechecker::file_to_str("tests/pa3_tests/enigma.eta");
    let expected = eta_typechecker::file_to_str("tests/pa3_tests/enigma.typed");
    assert_eq!(
        eta_typechecker::type_check_to_string(eta_typechecker::type_check(
            &input,
            "tests/pa3_tests/",
            "enigma.eta"
        )),
        expected
    );
}
