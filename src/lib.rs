#![warn(clippy::all)]
mod core_lib;
mod error;
mod parser;
mod runtime;
mod tokens;
mod value;

use error::{Position, Result};
use parser::parse_block;
use runtime::Runtime;
use tokens::Tokens;

use std::fs;

pub use error::OmgError;

pub fn run_file(file: &str) -> Result<()> {
    let source = load_file(&file)?;
    let mut tokens = Tokens::lex(&source, file.to_string())?;
    let exp = parse_block(&mut tokens)?;
    let mut runtime = Runtime::new();
    runtime.run(&exp);
    Ok(())
}

fn load_file(file: &str) -> Result<String> {
    match fs::read_to_string(file) {
        Ok(s) => Ok(s),
        Err(err) => Err(OmgError::new(
            err.to_string(),
            Position::new(file.to_string(), 0, 0),
        )),
    }
}
