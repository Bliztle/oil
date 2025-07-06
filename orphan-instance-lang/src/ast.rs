#[derive(Debug)]
pub struct Program {
    pub nodes: Vec<Definition>,
}

#[derive(Debug)]
pub enum Ident {
    Simple(String),
    Qualified(Vec<String>, String),
}

#[derive(Debug)]
pub enum Definition {
    Struct(Ident, Vec<StructField>),
    Trait(Ident, Vec<TraitMethod>),
    Impl(Impl),
    Mod(Ident, Vec<Definition>),
    Func(Function),
}

#[derive(Debug)]
pub struct StructField {
    pub ident: Ident,
    pub field_type: Type,
}

#[derive(Debug)]
pub struct TraitMethod {
    pub ident: Ident,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
}

#[derive(Debug)]
pub struct Parameter {
    pub ident: Ident,
    pub parameter_type: Type,
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Paren(Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Let(Ident, Option<Type>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Block(Vec<Expr>, Option<Box<Expr>>),
    Invocation(Ident, Vec<Expr>),
    Use(Ident, Box<Expr>),
    StructInit(Ident, Vec<(Ident, Expr)>),
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
    F32,
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
    pub ident: Ident,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Expr,
}
