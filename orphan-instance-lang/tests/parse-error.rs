use orphan_instance_lang::parser::parse;
use test_each_file::test_each_file;

test_each_file! { in "./orphan-instance-lang/tests/parse-error" => parse_error }
fn parse_error(content: &str) {
    assert!(parse(content).is_err());
}

test_each_file! { in "./orphan-instance-lang/tests/type-error" => parse_success }
fn parse_success(content: &str) {
    assert!(parse(content).is_ok());
}
