use extendr_api::*;

#[derive(Debug)]
pub enum Value {
    Float(f64),
    Int(i32),
    Str(&str),
    Null,
    NA
}
// Add NA? Maybe, because veen if it is a boolean, it is often used as a generic value

#[derive(Debug)]
pub enum Symbol {
    Sym(&str)
}

#[derive(Debug)]
pub enum Expr {
    Value,
    Symbol,
    Statements(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    For(Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>,),
    Repeat(Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    FunctionDef(Box<Expr>, Box<Expr>)
}




pub fn sexp_to_ast(sexp : Robj) -> Expr {
    match sexp.sexptype() {
        NILSXP => Expr::Null,
        SYMSXP => Expr::Symbol(sexp.as_str().unwrap()),
        _ => Expr::Null//...
    }
}