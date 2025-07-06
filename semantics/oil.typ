#import "@preview/fine-lncs:0.1.0": lncs, institute, author, theorem, proof

#let inst_aau = institute("Aalborg University", 
  // addr: "Princeton NJ 08544, USA"
  addr: "Fredrik Bajers Vej 7, 9220 Aalborg Øst, Denmark",
  email: "mail@bliztle.com"
  // url: "https://www.aau.dk/"
)

#show: lncs.with(
  title: "Orphan Instance Lang (OIL): Solving the Orphan Instance Problem",
  // thanks: "Supported by organization x.",
  authors: (
    author("Asbjørn Rysgaard Eriksen", 
      insts: (inst_aau),
      // oicd: "0000-0002-1234-5678",
    ),
  ),
  abstract: [
    This paper presents Orphan Instance Lang (OIL), a new programming language designed to A
    allieviate the challenges posed by orphan instances in functionally inspired 
    programming languages. OIL introduces the `use` keyword to disambiguate when multiple 
    implementations of a trait exist for a type, allowing programmers to specify which to
    use, when it cannot be inferred by the compiler. We also propose a semantic model which
    ensures stability in data structures which require consistent trait implementations.
  ],
  keywords: ("Orphan Instances", "Functional Programming", "Syntax", "Semantics", "Rust"),
  bibliography: bibliography("refs.bib")
)

= First Section

My awesome paper ...

= Syntax

#let Prog = "Prog"
#let Def = "Def"
#let Ident = "Ident"
#let QIdent = "QIdent"
#let TIdent = "TIdent"
#let Struct = "Struct"
#let Impl = "Impl"
#let Trait = "Trait"
#let Mod = "Mod"
#let Fn = "Fn"
#let Param = "Param"
#let ExprStat = "ExprStat"
#let Expr = "Expr"
#let ExprLet = "ExprLet"
#let ExprBlock = "ExprBlock"
#let ExprInvoke = "ExprInvoke"
#let ExprIf = "ExprIf"
#let ExprUse = "ExprUse"
#let ExprStructInit = "ExprStructInit"
#let Op = "Op"
#let Type = "Type"

Syntax is heavily inspired by Rust, with the addition of the `use` keyword to disambiguate
in the case of multiple, conflicting implementations of a trait for a type.
A program in OIL consists of a set of definitions (structs, traits, impls, and functions),
encapsulated in modules. Each of these definitions are simplified versions of their Rust counterparts.

$
  Prog ::= & Def^* \
  Def ::= & Mod \
    | & Struct \
    | & Impl \
    | & Trait \
    | & Fn \

  Mod ::= & "mod" Ident { Def^* } \

  Struct ::= & "struct" Ident { Struct'^* } \
  Struct' ::= & Ident : TIdent ;\

  Impl ::= & "impl" Ident { Fn^* }\
    | & "impl" Ident "for" Ident { Fn^* } \
    | & "impl" Ident "of" Ident "for" Ident { Fn^* } \

  Trait ::= & "trait" Ident { Trait'^* } \
  Trait' ::= & "fn" Ident "(" Param ")" "->" Ident \

  Fn ::= & "fn" Ident "(" Param ")" "->" TIdent { ExprStat } \
  Param ::= & Ident : Type \
    | & Param "," Ident : Type \
$

Ident is a simple identifier, QIdent is a qualified identifier (`intance.field`), and 
TIdent is a type identifier (`module::function`).

$
  Ident ::= & "[a-zA-Z_][a-zA-Z_0-9]*" \
  QIdent ::= & Ident \
    | & QIdent "." Ident \
  TIdent ::= & Ident \
    | & TIdent "::" Ident \
  Type ::= & TIdent \
    | & "()" \
    | & "i32" \
$

All statements in OIL are treated as expressions, with the exception of `let` expressions, 
which require a following expression to be valid. The only new addition is the `use` expression,
which allows the programmer to specify which implementation of a trait to use in the following 
expressions. `use` is also restricted to statements.

$
  ExprStat ::= & ExprLet \
    | & ExprUse \
    | & Expr \

  Expr ::= & Expr Op Expr' | Expr' \
  Expr' ::= & Expr' Op' Expr'' | Expr'' \
  Expr'' ::= & Expr'' Op'' Expr''' | Expr''' \
  Op ::= & "==" | "!=" | "<" \
    | & "<=" | ">" | ">=" \
  Op' ::= & "+" | "-" \
  Op'' ::= & "*" | "/" \
  Expr''' ::= & "[0-9]+" | "[0-9]*\.[0-9]+" \
    | & ( Expr ) \
    | & ExprBlock \
    | & ExprInvoke \
    | & ExprIf \
    | & QIdent \
  ExprLet ::= & "let" Ident "=" Expr \
  ExprBlock ::= & "{" ExprStat^* ";"? "}" \
  ExprInvoke ::= & QIdent "(" Expr^* ")" \
  ExprIf ::= & "if" Expr "then" ExprBlock \
    | & "if" Expr "then" ExprBlock "else" ExprBlock \
  ExprUse ::= & "use" TIdent "in" Expr \
  ExprStructInit ::= & QIdent "{" ExprStructInit' "}" \
  ExprStructInit' ::= & Ident ":" Expr \
    | & ExprStructInit' "," Ident ":" Expr \
$

== Syntactical Considerations

The curreny `use` syntax only allows `use` as expression statements, and thus do not allow
expressions like `1 + use Foo in bar(2)`. This was a trade-off between flexibility and verbosity,
as allowing `use` in expressions would require explicit blocks to disambiguate 
`use Foo in bar(2) + baz(3)`, becoming `use Foo in { bar(2) + baz(3) }`. This does however
lead to the more verbose let statement of `let x = { use Foo in bar(2) }`, which now require the
additional block around the outer expression. 

`if` uses `then` before the block as a disambiguation meassure. I would like to change this in 
the future, but have not found the time to do so yet. Currently, the grammar is not LR(1) without
it, as it cannot determine whether `if Ident {` is a ExprBlock or ExprStructInit.

= Semantics

= Implemention

== Limitations

The AST in the current implementation does not differentiate between qualified type identifiers, 
and qualified instance identifiers. Thus a situation may occur where referencing a function in
a module or impl which collide with an instance name may result in referencing the instance
instead. As an example, consider the following situation:

```oil
mod Foo {
  fn bar() -> i32 { 42 }
}

struct Baz {}

impl Baz {
  fn bar() -> i32 { 7 }
}

fn main() {
  let Foo = Baz {};
  // These will both call the instance method, not the module function
  Foo.bar(); 
  Foo::bar();
}
```

Here, both `Foo.bar()` and `Foo::bar()` are represented as `Ident::Qualified(["Foo"], "bar")`,
with no extra information. The easiest way to resolve this, is to use the existing AST annotation
system, and annotate qualified identifiers with the required information. This would not require
any modifications to the current AST.
