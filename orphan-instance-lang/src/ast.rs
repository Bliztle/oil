use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub oil
);

#[derive(Debug)]
pub struct Program {
    pub nodes: Vec<Definition>,
}

#[derive(Debug)]
pub struct Ident(String);

#[derive(Debug)]
pub enum Definition {
    Struct(Ident, Vec<StructField>),
    Trait(Ident, Vec<TraitMethod>),
    Impl(Impl),
    Mod,
    Func(Function),
}

#[derive(Debug)]
pub struct StructField {
    ident: Ident,
    field_type: Type,
}

#[derive(Debug)]
pub struct TraitMethod {
    ident: Ident,
    parameters: Vec<Parameter>,
    return_type: Type,
}

#[derive(Debug)]
pub struct Parameter {
    ident: Ident,
    parameter_type: Type,
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Paren(Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Let(Ident, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Invocation(Ident, Vec<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    I32(i32),
    F32(f32),
}

#[derive(Debug)]
pub enum BinOp {
    Mult,
    Div,
    Plus,
    Minus,
    LT,
    LTE,
    GT,
    GTE,
    Equal,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    I32,
    U32,
    Function(Vec<Parameter>, Box<Type>),
    Ident(Ident),
}

#[derive(Debug)]
pub enum Impl {
    /** Impl name, Trait name, Struct name */
    Trait(Option<Ident>, Ident, Ident, Vec<Function>),
    Struct(Ident, Vec<Function>),
}

#[derive(Debug)]
pub struct Function {
    ident: Ident,
    parameters: Vec<Parameter>,
    return_type: Option<Type>,
    body: Expr,
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
        assert!(bad.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_err()));
    }

    #[test]
    fn test_parse_impl() {
        let parser = oil::implParser::new();
        let func = "fn my_func() {}";

        let good = [
            format!("impl MyStruct {{ {func} }}"),
            format!("impl MyImpl of MyTrait for MyStruct {{ {func} }}"),
            format!("impl MyTrait for MyStruct {{ {func} }}"),
        ];

        let bad = [
            format!("impl {{ {func} }}"),
            format!("impl MyStruct MyStruct {{ {func} }}"),
            format!("impl MyImpl of for MyStruct {{ {func} }}"),
            format!("impl MyImpl MyTrait for MyStruct {{ {func} }}"),
        ];

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
        assert!(bad.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_err()));
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

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
        assert!(bad.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_err()));
    }

    #[test]
    fn test_parse_ident() {
        let parser = oil::identParser::new();

        let good = ["a1", "_a", "_", "kjdngf_34", "gdfg3"];

        let bad = [
            "1a", "sdf@f", // Keywords
            "fn", "trait", "impl", "for", "mod", // "if",
            // "else",
            "i32", "u32",
        ];

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
        assert!(bad.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_err()));
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
            "if 1 { 1 } else { 2 }",
            "if (3) { 1 } else { 2 }",
            "if (3 == 2) { 1 } else { 2 }",
            "if 3 > 2 { 1 } else { 2 }",
        ];

        let bad = ["543.", "4s", "let 2 = 3", "1 2", "1,2", "{ 1 2 }"];

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
        assert!(bad.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_err()));
    }

    #[test]
    fn test_parse_expr_statement() {
        let parser = oil::expr_statementParser::new();

        let good = [
            "if 1 { 1 } else { 2 }",
            "if (3) { 1 } else { 2 }",
            "if (3 == 2) { 1 } else { 2 }",
            "if 3 > 2 { 1 } else { 2 }",
            "let x = 3",
        ];

        let bad = [
            "1",
            "324908",
            "0.654",
            "234.9",
            "a",
            "fdsf3",
            "1 + 2",
            "( 1 )",
            "( 1 + 2 )",
        ];

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
        assert!(bad.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_err()));
    }
}
