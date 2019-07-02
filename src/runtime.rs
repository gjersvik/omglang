mod scope;

use crate::parser::Exp;
use scope::Scope;

use std::{iter::FromIterator, sync::Arc};

pub enum Value {
    Nothing,
    UInt(u64),
    Function(Box<dyn Fn(&[Value]) -> Value>),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::UInt(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
            Value::Function(_) => "BuiltIn function".to_string(),
        }
    }
}

pub struct Runtime {
    local: Scope,
}

impl Runtime {
    pub fn new() -> Runtime {
        let g = global();
        Runtime {
            local: Scope::parent(Some(g)),
        }
    }

    pub fn run(&mut self, exp: &Exp) -> Value {
        self.run_exp(exp)
    }

    fn run_exp(&mut self, exp: &Exp) -> Value {
        match exp {
            Exp::Call(i, args) => {
                let v = self.local.get(i);
                match *v {
                    Value::Function(ref function) => function(&self.run_list(args)),
                    _ => panic!("Cant find function named {} to call", i),
                }
            }
            Exp::Block(block) => {
                self.run_list(&block);
                Value::Nothing
            }
            Exp::LiteralUInt(int) => Value::UInt(*int),
            Exp::InValid => panic!("Invalid expression found."),
        }
    }

    fn run_list(&mut self, exps: &[Exp]) -> Vec<Value> {
        let iter = exps.iter().map(|exp| self.run_exp(exp));
        Vec::from_iter(iter)
    }
}

fn print(args: &[Value]) -> Value {
    let string = Vec::from_iter(args.iter().map(|v| v.to_string())).join(" ");
    println!("{}", string);
    Value::Nothing
}

fn global() -> Arc<Scope> {
    let mut scope = Scope::new();
    scope.set(
        "print".to_string(),
        Arc::new(Value::Function(Box::new(print))),
    );
    Arc::new(scope)
}
