use super::runtime::scope::Scope;
use crate::value::Value;

use std::{iter::FromIterator, sync::Arc};

fn print(args: &[Value]) -> Value {
    let string = Vec::from_iter(args.iter().map(|v| v.to_string())).join(" ");
    println!("{}", string);
    Value::Nothing
}

pub fn global() -> Arc<Scope> {
    let mut scope = Scope::new();
    scope.set(
        "print".to_string(),
        Arc::new(Value::Function(Arc::new(&print))),
    );
    Arc::new(scope)
}
