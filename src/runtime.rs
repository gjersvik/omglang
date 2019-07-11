use im::Vector;

use super::{
    error::{OmgError, Result},
    parser::Exp,
    value::{Value, Scope},
};


pub struct Runtime {
    local: Scope,
}

impl Runtime {
    pub fn new(global: &Scope) -> Runtime {
        Runtime { local: global.clone() }
    }

    pub fn run(&mut self, exp: &Exp) -> Result<Value> {
        self.run_exp(exp)
    }

    fn run_exp(&mut self, exp: &Exp) -> Result<Value> {
        match &exp {
            Exp::Call(call) => {
                let v = self
                    .local
                    .get(&call.name)
                    .unwrap_or(&Value::Nothing)
                    .clone();
                match v {
                    Value::NativeFunction(native) => Ok(native.call(self.run_list(&call.args)?)),
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

    fn run_list(&mut self, expressions: &[Exp]) -> Result<Vector<Value>> {
        expressions.iter().map(|exp| self.run_exp(exp)).collect()
    }
}
