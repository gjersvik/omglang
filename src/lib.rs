#![warn(clippy::all)]
mod parser;
mod runtime;
mod tokens;

use tokens::Tokens;
use parser::parse_block;
use runtime::run_exp;

pub fn run(code: &str) {
    let mut tokens = Tokens::lex(code);
    println!("Tokens: {:?}", tokens);
    let exp = parse_block(&mut tokens);
    println!("Exp: {:?}", exp);
    println!("Running program:");
    run_exp(&exp);
    println!("Program done");
}
