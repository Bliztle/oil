use crate::ast::Program;
use lalrpop_util::lalrpop_mod;
use regex::Regex;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub oil
);

#[derive(Debug)]
pub struct ParseError;

#[allow(clippy::missing_panics_doc)]
pub fn parse(input: &str) -> Result<Program, ParseError> {
    let input = remove_comments(input);
    dbg!(&input);

    let parser = oil::programRuleParser::new();
    match dbg!(parser.parse(&input)) {
        Ok(program) => Ok(program),
        // TODO: Generate proper error message
        Err(_) => Err(ParseError),
    }
}

fn remove_comments(input: &str) -> std::borrow::Cow<'_, str> {
    let re = Regex::new(r"//[^\n]*|/\*.*\*/").unwrap();
    re.replace_all(input, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_good {
        ($strs:expr, $parser:expr) => {
            let mut success = true;
            for s in $strs {
                let res = $parser.parse(s);
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(s);
                        dbg!("Expected Ok, but got error");
                        dbg!(e);
                        success = false;
                    }
                }
            }
            assert!(success);
        };
    }

    macro_rules! assert_bad {
        ($strs:expr, $parser:expr) => {
            let mut success = true;
            for s in $strs {
                let res = $parser.parse(s);
                match res {
                    Ok(_) => {
                        dbg!(s);
                        dbg!("Expected error, but got Ok");
                        success = false;
                    }
                    Err(_) => {}
                }
            }
            assert!(success);
        };
    }

    #[test]
    fn test_parse_func() {
        let parser = oil::funcParser::new();

        let good = [
            "fn my_function () {}",
            "fn my_function ( ) { }",
            "fn my_function (a: A) { a }",
            "fn my_function (a: A) -> A { a }",
        ];

        let bad = [
            "fn my_function my_function () {}",
            "fn my_function ( } {)",
            "fn my_function (a: A) { fn }",
            "fn my_function (a A) { }",
            "fn my_function (a) { }",
        ];

        assert_good!(good, parser);
        assert_bad!(bad, parser);
    }

    #[test]
    fn test_parse_impl() {
        let parser = oil::implParser::new();
        let func = "fn my_func() {}";

        let good = [
            &format!("impl MyStruct {{ {func} }}"),
            &format!("impl MyImpl of MyTrait for MyStruct {{ {func} }}"),
            &format!("impl MyTrait for MyStruct {{ {func} }}"),
        ];

        let bad = [
            &format!("impl {{ {func} }}"),
            &format!("impl MyStruct MyStruct {{ {func} }}"),
            &format!("impl MyImpl of for MyStruct {{ {func} }}"),
            &format!("impl MyImpl MyTrait for MyStruct {{ {func} }}"),
        ];

        assert_good!(good, parser);
        assert_bad!(bad, parser);
    }

    #[test]
    fn test_parse_trait() {
        let parser = oil::traitParser::new();

        let good = [
            "trait MyTrait {}",
            "trait MyTrait { }",
            "trait MyTrait { fn my_func() -> A }",
            "trait MyTrait { fn my_func(a: A) -> A }",
            "trait MyTrait { fn my_func(a: A) -> () }",
        ];

        let bad = ["trait trait", "trait MyTrait"];

        assert_good!(good, parser);
        assert_bad!(bad, parser);
    }

    #[test]
    fn test_parse_ident() {
        let parser = oil::identParser::new();

        let good = ["a1", "_a", "_", "kjdngf_34", "gdfg3"];

        let bad = [
            "1a", "sdf@f", "fn", "trait", "impl", "for", "mod", "if", "else", "i32",
        ];

        assert_good!(good, parser);
        assert_bad!(bad, parser);
    }

    #[test]
    fn test_parse_expr() {
        let parser = oil::exprParser::new();

        let good = [
            "1",
            "324908",
            "0.654",
            "234.9",
            "a",
            "fdsf3",
            "1 + 2",
            "{ 1 }",
            "( 1 )",
            "( 1 + 2 )",
            "if 1 then { 1 }",
            "if 1 then { 1 } else { 2 }",
            "if (3) then { 1 } else { 2 }",
            "if (3 == 2) then { 1 } else { 2 }",
            "if 3 > 2 then { 1 } else { 2 }",
            "if MyStruct { a: 1, b: 2 } == myStruct then { 1 } else { 2 }",
            "{ call(x) }",
            "{ call(x); }",
            "{ use A in call(x) }",
            "{ use A in call(x); }",
            "{ use A in { call(x) } }",
            "{ use A in { call(x); } }",
            "{ use A in { call(x) }; }",
            "{ use A in { call(x); }; }",
        ];

        let bad = [
            "543.",
            "4s",
            "let 2 = 3",
            "1 2",
            "1,2",
            "{ 1 2 }",
            "if 1 then { 2 } else",
            "if 1 { 2 }",
        ];

        assert_good!(good, parser);
        assert_bad!(bad, parser);
    }

    #[test]
    fn test_parse_expr_statement() {
        let parser = oil::expr_statementParser::new();

        let good = [
            "if 1 then { 1 } else { 2 }",
            "if (3) then { 1 } else { 2 }",
            "if (3 == 2) then { 1 } else { 2 }",
            "if 3 > 2 then { 1 } else { 2 }",
            "let x = 3",
            "let x: i32 = 3",
            "let MyStruct = myStruct",
            "let MyStruct = MyStruct { a: 1, b: 2 }",
        ];

        let bad = ["1d"];

        assert_good!(good, parser);
        assert_bad!(bad, parser);
    }

    #[test]
    fn test_remove_comments() {
        let input = r"
// Comment here
// In multiple lines
struct A { }

// And now
let x = /* an inline */ 3; // comment
        ";
        let output = r"


struct A { }


let x =  3; 
        ";

        assert_eq!(remove_comments(input), output);
    }
}
