#![warn(clippy::all)]
mod core_lib;
mod error;
mod parser;
mod runtime;
mod tokens;
mod value;

use core_lib::global;
use error::{Position, Result};
use parser::parse_block;
use runtime::Runtime;
use tokens::Tokens;
use value::Scope;

use std::fs;

pub use error::OmgError;

#[macro_use]
extern crate im;

pub struct OmgLang {
    global: Scope,
}

impl OmgLang {
    pub fn new() -> Self {
        OmgLang { global: global() }
    }

    pub fn run_file(&self, file: &str) -> Result<()> {
        let source = OmgLang::load_file(&file)?;
        let mut tokens = Tokens::lex(&source, file)?;
        let exp = parse_block(&mut tokens)?;
        let mut runtime = Runtime::new(&self.global);
        runtime.run(&exp)?;
        Ok(())
    }

    fn load_file(file: &str) -> Result<String> {
        match fs::read_to_string(file) {
            Ok(s) => Ok(s),
            Err(err) => Err(OmgError::new(err.to_string(), Position::new(file))),
        }
    }
}
