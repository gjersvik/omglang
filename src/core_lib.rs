use im::Vector;

use crate::value::{Scope, Value};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Native {
    Print,
}

impl Native {
    pub fn call(&self, args: Vector<Value>) -> Value {
        match self {
            Native::Print => print(args),
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
fn print(args: Vector<Value>) -> Value {
    let string = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", string);
    Value::Nothing
}

pub fn global() -> Scope {
    hashmap! {
        "print".to_string() => Value::NativeFunction(Native::Print)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_test() {
        let global = global();
        assert_eq!(
            *global.get("print").unwrap(),
            Value::NativeFunction(Native::Print)
        );
    }
}
