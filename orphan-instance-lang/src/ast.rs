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
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Let(Ident, Box<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    I32(i32),
    U32(u32),
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
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
    body: Vec<Expr>,
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

        assert!(good.iter().all(|s| dbg!(parser.parse(dbg!(s))).is_ok()));
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

        assert!(good
            .into_iter()
            .all(|s| dbg!(parser.parse(dbg!(&s))).is_ok()));
        assert!(bad.into_iter().all(|s| dbg!(parser.parse(&s)).is_err()));
    }

    #[test]
    fn test_parse_trait() {
        let parser = oil::traitParser::new();
        let res = parser.parse("trait aname");
        println!("{res:?}");
        assert!(res.is_ok());
    }
}
