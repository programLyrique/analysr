use extendr_api::*;

#[derive(Debug)]
pub enum Value {
    Real(f64),
    Int(i32),
    Str(String),
    Bool(bool),
    Null,
    NA // Or one NA per type? Or Na(NA) where NA is another Enum
}
// Add NA? Maybe, because veen if it is a boolean, it is often used as a generic value

#[derive(Debug)]
pub enum Symbol {
    Sym(String)
}

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Symbol(Symbol),
    Statements(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    For(Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>,),
    Repeat(Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    FunctionDef(Box<Expr>, Box<Expr>)
}




pub fn sexp_to_ast(sexp : Robj) -> Expr {
    //println!("Sexptype: {:?}", sexp);
    match sexp.rtype() {
        RType::Integer | RType::Logical | RType::Real | RType::Complex | RType::String if sexp.is_na() => Expr::Value(Value::NA),
        RType::Integer => Expr::Value(Value::Int(sexp.as_integer().unwrap())),
        RType::Real => Expr::Value(Value::Real(sexp.as_real().unwrap())),
        RType::Logical => Expr::Value(Value::Bool(sexp.as_bool().unwrap())),
        RType::String => Expr::Value(Value::Str(sexp.as_str().unwrap().to_string())),
        RType::Null => Expr::Value(Value::Null),
        RType::Symbol => Expr::Symbol(Symbol::Sym(sexp.as_symbol().unwrap().0.to_string())),
        RType::Language => {
            let mut lang = sexp.as_pairlist_iter().unwrap();
            let func_name = Box::new(sexp_to_ast(lang.next().unwrap()));
            let args = lang.map(sexp_to_ast).collect();
            Expr::Call(func_name, args)
        }
        RType::Expression => {
            Expr::Statements(sexp.as_list_iter().unwrap().map(sexp_to_ast).collect())
        }
        _ => Expr::Value(Value::Null)//...
    }
}