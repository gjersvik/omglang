pub mod scope;
pub mod value;

use super::{core_lib::global, parser::Exp};
use scope::Scope;
use value::Value;

use std::iter::FromIterator;

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
