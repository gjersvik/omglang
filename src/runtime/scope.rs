use super::Value;

use std::collections::HashMap;
use std::sync::Arc;

pub struct Scope {
    values: HashMap<String, Arc<Value>>,
    parent: Option<Arc<Scope>>,
}

impl Scope {
    pub fn get(&self, name: &str) -> Arc<Value> {
        match self.values.get(name) {
            Some(v) => v.clone(),
            None => match &self.parent {
                Some(p) => p.get(name),
                None => Arc::new(Value::Nothing),
            },
        }
    }

    pub fn set(&mut self, name: String, value: Arc<Value>) {
        self.values.insert(name, value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
