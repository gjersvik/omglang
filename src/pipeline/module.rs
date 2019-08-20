use crate::pipeline::Function;
use im::HashMap;

pub struct Module {
    functions: HashMap<String, Function>,
}

impl Module {
    pub fn new() -> Self {
        Module {
            functions: HashMap::new(),
        }
    }

    pub fn add_function<S>(&self, name: S, function: Function) -> Self
    where
        S: Into<String>,
    {
        Module {
            functions: self.functions.update(name.into(), function),
        }
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        self.functions.get(name).cloned()
    }
}
