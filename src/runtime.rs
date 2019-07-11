pub mod scope;

use super::{
    core_lib::global,
    error::{OmgError, Result},
    parser::Exp,
    value::Value,
};
use scope::Scope;

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

    pub fn run(&mut self, exp: &Exp) -> Result<Value> {
        self.run_exp(exp)
    }

    fn run_exp(&mut self, exp: &Exp) -> Result<Value> {
        match &exp {
            Exp::Call(call) => {
                let v = self.local.get(&call.name);
                match *v {
                    Value::Function(ref function) => Ok(function(&self.run_list(&call.args)?)),
                    _ => Err(OmgError::new(
                        format!("Cant find function named {} to call", call.name),
                        call.pos.clone(),
                    )),
                }
            }
            Exp::Block(block) => {
                self.run_list(&block.statements)?;
                Ok(Value::Nothing)
            }
            Exp::Literal(literal) => Ok(literal.value.clone()),
        }
    }

    fn run_list(&mut self, expressions: &[Exp]) -> Result<Vec<Value>> {
        expressions.iter().map(|exp| self.run_exp(exp)).collect()
    }
}
