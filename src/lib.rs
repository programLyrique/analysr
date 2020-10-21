use std::fs;

use extendr_api::*;





#[extendr]
pub fn parse_file(filename: &str) -> Robj
{
    let code = fs::read_to_string(filename).unwrap();

    let p = Robj::parse("print(hello)").unwrap();

    return p
}