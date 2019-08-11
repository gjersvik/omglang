use crate::function::Function;
use im::HashMap;

pub struct ModuleScope {
    functions: HashMap<String, Function>,
}

impl ModuleScope {
    pub fn new() -> Self {
        ModuleScope {
            functions: HashMap::new(),
        }
    }

    pub fn add_function<S>(&self, name: S, function: Function) -> Self
    where
        S: Into<String>,
    {
        ModuleScope {
            functions: self.functions.update(name.into(), function),
        }
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        self.functions.get(name.into()).cloned()
    }
}
