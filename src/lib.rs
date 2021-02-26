use std::fs;

use extendr_api::*;
use extendr_api::functions::parse;

mod rast;
mod simplify;
mod counters;


pub fn parse_file(filename: &str) -> rast::Expr
{
    let code = fs::read_to_string(filename).unwrap();

    let p = parse(&code).unwrap();

    return rast::sexp_to_ast(p)
}