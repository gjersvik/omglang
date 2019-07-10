pub mod scope;
pub mod value;

use super::{
    core_lib::global,
    parser::{Exp, ExpValue},
};
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
        match &exp.value {
            ExpValue::Call(i, args) => {
                let v = self.local.get(i);
                match *v {
                    Value::Function(ref function) => function(&self.run_list(args)),
                    _ => panic!("Cant find function named {} to call", i),
                }
            }
            ExpValue::Block(block) => {
                self.run_list(&block);
                Value::Nothing
            }
            ExpValue::LiteralUInt(int) => Value::UInt(*int),
        }
    }

    fn run_list(&mut self, expressions: &[Exp]) -> Vec<Value> {
        let iter = expressions.iter().map(|exp| self.run_exp(exp));
        Vec::from_iter(iter)
    }
}
