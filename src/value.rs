use im::HashMap;

use crate::core_lib::Native;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Nothing,
    Int(i64),
    NativeFunction(Native),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
            Value::NativeFunction(_) => "BuiltIn function".to_string(),
        }
    }
}

pub type Scope = HashMap<String, Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_to_string() {
        assert_eq!(Value::Nothing.to_string(), "Nothing")
    }

    #[test]
    fn int_to_string() {
        assert_eq!(Value::Int(42).to_string(), "42")
    }

    #[test]
    fn function_to_string() {
        assert_eq!(
            Value::NativeFunction(Native::Print).to_string(),
            "BuiltIn function"
        )
    }
}
