use orphan_instance_lang::ast::oil;
use test_each_file::test_each_file;

test_each_file! { in "./orphan-instance-lang/tests/parse-error" => parse_error }
fn parse_error(content: &str) {
    let parser = oil::programRuleParser::new();
    assert!(dbg!(parser.parse(content)).is_err());
}

test_each_file! { in "./orphan-instance-lang/tests/type-error" => parse_success }
// test_each_file! { in "./orphan-instance-lang/tests/run-error" => parse_sucess }
// test_each_file! { in "./orphan-instance-lang/tests/run-success" => parse_error }
fn parse_success(content: &str) {
    let parser = oil::programRuleParser::new();
    assert!(dbg!(parser.parse(content)).is_ok());
}
