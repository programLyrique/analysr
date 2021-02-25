use extendr_api::*;

#[derive(Debug, PartialEq)]
pub enum Value {
    Real(f64),
    Int(i32),
    Str(String),
    Bool(bool),
    Null,
    NA, // Or one NA per type? Or Na(NA) where NA is another Enum
}
// Add NA? Maybe, because even if it is a boolean, it is often used as a generic value


// TODO: add BinaryOp?
#[derive(Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Symbol(String),
    Statements(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>),
    Repeat(Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    FunctionDef(Box<Expr>, Box<Expr>),
    ArgList(Vec<String>),
    Break,
    Next,
    Empty, //Rather use an Option type?
}

pub fn sexp_to_ast(sexp: Robj) -> Expr {
    //println!("Sexptype: {:?}", sexp);
    match sexp.rtype() {
        RType::Integer | RType::Logical | RType::Real | RType::Complex | RType::String
            if sexp.is_na() =>
        {
            Expr::Value(Value::NA)
        }
        RType::Integer => Expr::Value(Value::Int(sexp.as_integer().unwrap())),
        RType::Real => Expr::Value(Value::Real(sexp.as_real().unwrap())),
        RType::Logical => Expr::Value(Value::Bool(sexp.as_bool().unwrap())),
        RType::String => Expr::Value(Value::Str(sexp.as_str().unwrap().to_string())),
        RType::Null => Expr::Value(Value::Null),
        RType::Symbol => Expr::Symbol(sexp.as_symbol().unwrap().0.to_string()),
        RType::Language => {
            let mut lang = sexp.as_pairlist_iter().unwrap();
            let func_name = sexp_to_ast(lang.next().unwrap());
            let mut args = lang.map(sexp_to_ast).collect::<Vec<_>>();

            if let Expr::Symbol(ref s) = func_name {
                match s.as_str() {
                    "function" => {
                        // drain or swap-remove or into_inter?
                        let mut args_drain = args.drain(0..1); // First argument is a src ref. We do not care about it
                        let arg_list = args_drain.next().unwrap();
                        let body = args_drain.next().unwrap();
                        Expr::FunctionDef(Box::new(arg_list), Box::new(body))
                    }
                    "if" => {
                        let mut args_drain = args.drain(..);
                        let cond = args_drain.next().unwrap();
                        let body1 = args_drain.next().unwrap();
                        let body2 = args_drain.next().unwrap_or(Expr::Empty); //For if without an else block
                        Expr::If(Box::new(cond), Box::new(body1), Box::new(body2))
                    }
                    "while" => {
                        let mut args_drain = args.drain(..);
                        let cond = args_drain.next().unwrap();
                        let body = args_drain.next().unwrap();
                        Expr::While(Box::new(cond), Box::new(body))
                    }
                    "for" => {
                        let mut args_drain = args.drain(..);
                        let iter_var = if let Expr::Symbol(v) = args_drain.next().unwrap() {
                            v
                        } else {
                            unreachable!();
                        };
                        let seq = args_drain.next().unwrap();
                        let body = args_drain.next().unwrap();
                        Expr::For(iter_var, Box::new(seq), Box::new(body))
                    }
                    "repeat" => {
                        let body = args.drain(..).next().unwrap();
                        Expr::Repeat(Box::new(body))
                    }
                    "{" => Expr::Statements(args),
                    "break" => Expr::Break,
                    "next" => Expr::Next,
                    _ => Expr::Call(Box::new(func_name), args),
                }
            } else {
                Expr::Call(Box::new(func_name), args) //transform to an anonymous function def?
            }
        }
        RType::Expression => {
            let mut statements = sexp
                .as_list_iter()
                .unwrap()
                .map(sexp_to_ast)
                .collect::<Vec<_>>();
            // Simplify if there are only one statement in an expression
            if statements.len() == 1 {
                statements.remove(0)
            } else {
                Expr::Statements(statements)
            }
        }

        RType::Pairlist => {
            // Function arguments
            Expr::ArgList(
                sexp.as_pairlist_tag_iter()
                    .unwrap()
                    .map(|arg| arg.to_string())
                    .collect(),
            )
        }
        _ => panic!(format!("Unsupported: {:?}\n", sexp)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctor::*;
    use extendr_api::functions::parse;
    use extendr_engine::*;

    #[ctor]
    fn init() {
        // It will be initialized before any thread creation
        // We need it because we can only start R once, it is not reentrant.
        start_r();
    }

    #[test]
    fn values() {
        assert_eq!(sexp_to_ast(R!(NA).unwrap()), super::Expr::Value(Value::NA));
        assert_eq!(
            sexp_to_ast(R!(1L).unwrap()),
            super::Expr::Value(Value::Int(1))
        );
        assert_eq!(
            sexp_to_ast(R!(1).unwrap()),
            super::Expr::Value(Value::Real(1.0))
        );
        assert_eq!(
            sexp_to_ast(R!("hello world").unwrap()),
            super::Expr::Value(Value::Str("hello world".to_string()))
        )
    }

    #[test]
    fn symbol() {
        assert_eq!(
            sexp_to_ast(parse("x").unwrap()),
            super::Expr::Symbol("x".to_string())
        );
    }

    #[test]
    fn controls() {
        assert_eq!(
            format!("{:?}", sexp_to_ast(parse("if(x == 1) 1 else 2").unwrap())),
            "If(Call(Symbol(\"==\"), [Symbol(\"x\"), Value(Real(1.0))]), Value(Real(1.0)), Value(Real(2.0)))");
        
            assert_eq!(
            format!("{:?}", sexp_to_ast(parse("while(1) { print(\"hello\") }").unwrap())),   
            "While(Value(Real(1.0)), Statements([Call(Symbol(\"print\"), [Value(Str(\"hello\"))])]))" );

        assert_eq!(
            format!("{:?}", sexp_to_ast(parse("for(i in 1:10) { i + 5 }").unwrap())), 
            "For(\"i\", Call(Symbol(\":\"), [Value(Real(1.0)), Value(Real(10.0))]), Statements([Call(Symbol(\"+\"), [Symbol(\"i\"), Value(Real(5.0))])]))"
        );

        assert_eq!(
            format!("{:?}", sexp_to_ast(parse("repeat { 1 + 1 }").unwrap())),
            "Repeat(Statements([Call(Symbol(\"+\"), [Value(Real(1.0)), Value(Real(1.0))])]))"
        )
    }
}
