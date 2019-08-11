#![warn(clippy::all)]
mod core_lib;
mod error;
mod function;
mod module_scope;
mod parser;
mod runtime;
mod tokens;
mod value;

use crate::core_lib::add_std_lib;
use crate::module_scope::ModuleScope;
use error::{Position, Result};
use parser::parse_block;
use runtime::Runtime;
use tokens::Tokens;

use std::fs;
use std::sync::Arc;

pub use error::OmgError;

pub struct OmgLang {
    module: Arc<ModuleScope>,
}

impl OmgLang {
    pub fn new() -> Self {
        let module = ModuleScope::new();
        let module = add_std_lib(&module);
        OmgLang {
            module: Arc::new(module),
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn run_file(&self, file: &str) -> Result<()> {
        let source = OmgLang::load_file(&file)?;
        let mut tokens = Tokens::lex(&source, file)?;
        let exp = parse_block(&mut tokens)?;
        let mut runtime = Runtime::new(&self.module);
        runtime.run(&exp)?;
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn load_file(file: &str) -> Result<String> {
        match fs::read_to_string(file) {
            Ok(s) => Ok(s),
            Err(err) => Err(OmgError::new(err.to_string(), Position::new(file))),
        }
    }
}

#[cfg(test)]
mod tests {}
