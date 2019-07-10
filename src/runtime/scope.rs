use super::Value;

use std::collections::HashMap;
use std::sync::Arc;

pub struct Scope {
    values: HashMap<String, Arc<Value>>,
    parent: Option<Arc<Scope>>,
}

impl Scope {
    pub fn new() -> Scope {
        Self::parent(Option::None)
    }

    pub fn parent(parent: Option<Arc<Scope>>) -> Scope {
        Scope {
            values: HashMap::new(),
            parent,
        }
    }

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
    use super::*;

    #[test]
    fn get_missing() {
        let scope = Scope::new();
        assert_eq!(*scope.get("missing"), Value::Nothing);
    }

    #[test]
    fn set_get_value() {
        let mut scope = Scope::new();
        let value = Arc::new(Value::Int(42));
        scope.set("life".to_string(), value.clone());

        assert_eq!(scope.get("life"), value);
    }

    #[test]
    fn get_value_from_parent() {
        let mut scope = Scope::new();
        let value = Arc::new(Value::Int(42));
        scope.set("life".to_string(), value.clone());
        let scope = Scope::parent(Some(Arc::new(scope)));
        assert_eq!(scope.get("life"), value);
    }

    #[test]
    fn get_missing_from_parent() {
        let scope = Scope::new();
        let scope = Scope::parent(Some(Arc::new(scope)));
        assert_eq!(*scope.get("missing"), Value::Nothing);
    }

    #[test]
    fn local_overwrite_parent() {
        let mut scope = Scope::new();
        scope.set("life".to_string(), Arc::new(Value::Int(126)));
        let mut scope = Scope::parent(Some(Arc::new(scope)));
        let value = Arc::new(Value::Int(42));
        scope.set("life".to_string(), value.clone());
        assert_eq!(scope.get("life"), value);
    }
}
