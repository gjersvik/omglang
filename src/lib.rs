#![warn(clippy::all)]
mod core_lib;
mod error;
mod function;
mod module_scope;
mod pipeline;
mod runtime;
mod value;

use crate::core_lib::add_std_lib;
use crate::module_scope::ModuleScope;
use pipeline::parse_block;
use runtime::Runtime;
use tokio::prelude::Future;

use std::sync::Arc;

pub use error::OmgError;

pub struct OmgLang {
    module: Arc<ModuleScope>,
}

impl Default for OmgLang {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn run_file(&self, file: &str) -> impl Future<Item = (), Error = OmgError> {
        let module = Arc::clone(&self.module);
        pipeline::loader(file.to_string()).and_then(move |source| {
            let mut tokens = pipeline::lexer(source)?;
            let exp = parse_block(&mut tokens)?;
            let mut runtime = Runtime::new(&module);
            runtime.run(&exp)?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {}
