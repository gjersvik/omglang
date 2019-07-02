#![warn(clippy::all)]
mod parser;
mod runtime;
mod tokens;

use parser::parse_block;
use runtime::Runtime;
use tokens::Tokens;

pub fn run(code: &str) {
    let mut tokens = Tokens::lex(code);
    println!("Tokens: {:?}", tokens);
    let exp = parse_block(&mut tokens);
    println!("Exp: {:?}", exp);
    println!("Running program:");
    let mut runtime = Runtime::new();
    runtime.run(&exp);
    println!("Program done");
}
