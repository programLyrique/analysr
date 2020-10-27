use std::fs;

use extendr_api::*;

mod rast;


#[extendr]
pub fn parse_file(filename: &str) -> rast::Expr
{
    let code = fs::read_to_string(filename).unwrap();

    let p = Robj::parse(&code).unwrap();

    return rast::sexp_to_ast(p)
}