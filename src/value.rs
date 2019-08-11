use im::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Nothing,
    Number(f64),
    True,
    False,
}

impl Value {
    pub fn from_bool(v: bool) -> Value {
        match v {
            true => Value::True,
            false => Value::False,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
            Value::True => "True".to_string(),
            Value::False => "False".to_string(),
        }
    }

    pub fn add(&self, other: &Value) -> Value {
        let values = (self, other);
        match values {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => Value::Nothing,
        }
    }

    pub fn subtract(&self, other: &Value) -> Value {
        let values = (self, other);
        match values {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Value::Nothing,
        }
    }

    pub fn multiply(&self, other: &Value) -> Value {
        let values = (self, other);
        match values {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Value::Nothing,
        }
    }

    pub fn divide(&self, other: &Value) -> Value {
        let values = (self, other);
        match values {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Value::Nothing,
        }
    }

    pub fn equal(&self, other: &Value) -> Value {
        Value::from_bool(self == other)
    }

    pub fn greater_than(&self, other: &Value) -> Value {
        let values = (self, other);
        match values {
            (Value::Number(a), Value::Number(b)) => Value::from_bool(a > b),
            _ => Value::Nothing,
        }
    }

    pub fn less_than(&self, other: &Value) -> Value {
        let values = (self, other);
        match values {
            (Value::Number(a), Value::Number(b)) => Value::from_bool(a < b),
            _ => Value::Nothing,
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
    fn number_to_string() {
        assert_eq!(Value::Number(42.0).to_string(), "42")
    }

    #[test]
    fn true_to_string() {
        assert_eq!(Value::True.to_string(), "True")
    }

    #[test]
    fn false_to_string() {
        assert_eq!(Value::False.to_string(), "False")
    }

    #[test]
    fn add_numbers() {
        assert_eq!(
            Value::Number(5.0).add(&Value::Number(10.0)),
            Value::Number(15.0)
        )
    }

    #[test]
    fn add_wrong_type() {
        assert_eq!(Value::Number(5.0).add(&Value::Nothing), Value::Nothing)
    }
}
