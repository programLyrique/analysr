use std::env;

use analysr::*;

use extendr_api::*;

fn main() {

    start_r();// Compulsory to have here (otherwise segmentation fault when parsing)

    let args : Vec<String> = env::args().collect();

    println!("Reading file {}", args[1]);

    let p = analysr::parse_file(&args[1]);

    end_r();// We don't need the interpreter anymore here

    println!("\n{:?}", p);

}
