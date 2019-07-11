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
