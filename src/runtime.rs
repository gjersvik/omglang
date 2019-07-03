mod scope;

use crate::parser::Exp;
use scope::Scope;

use std::{cmp, fmt, iter::FromIterator, sync::Arc};

#[derive(Debug, PartialEq)]
pub enum Value {
    Nothing,
    UInt(u64),
    Function(Box<Arc<OmgFn>>),
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

    fn run_list(&mut self, expressions: &[Exp]) -> Vec<Value> {
        let iter = expressions.iter().map(|exp| self.run_exp(exp));
        Vec::from_iter(iter)
    }
}

pub trait OmgFnTr: Fn(&[Value]) -> Value {}
impl<F> OmgFnTr for F where F: Fn(&[Value]) -> Value + Copy {}
pub type OmgFn = dyn OmgFnTr<Output = Value>;

impl fmt::Debug for OmgFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OmgFn")
    }
}

impl cmp::PartialEq for OmgFn {
    fn eq(&self, other: &Self) -> bool {
        self == other
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
        Arc::new(Value::Function(Box::new(Arc::new(print)))),
    );
    Arc::new(scope)
}
