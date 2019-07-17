use im::Vector;

use super::{
    error::{OmgError, Result},
    parser::{Exp, OpType, Operator},
    value::{Scope, Value},
};

pub struct Runtime {
    local: Scope,
}

impl Runtime {
    pub fn new(global: &Scope) -> Runtime {
        Runtime {
            local: global.clone(),
        }
    }

    pub fn run(&mut self, exp: &Exp) -> Result<Value> {
        self.run_exp(exp)
    }

    fn run_exp(&mut self, exp: &Exp) -> Result<Value> {
        match exp {
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
            Exp::Assignment(assignment) => {
                let value = self.run_exp(&assignment.value)?;
                self.local.insert(assignment.name.clone(), value);
                Ok(Value::Nothing)
            }
            Exp::Variable(variable) => Ok(self
                .local
                .get(&variable.name)
                .unwrap_or(&Value::Nothing)
                .clone()),
            Exp::Operator(op) => self.run_operator(op),
        }
    }

    fn run_list(&mut self, expressions: &[Exp]) -> Result<Vector<Value>> {
        expressions.iter().map(|exp| self.run_exp(exp)).collect()
    }

    fn run_operator(&mut self, op: &Operator) -> Result<Value> {
        let lhs = self.run_exp(&op.lhs)?;
        let rhs = self.run_exp(&op.rhs)?;
        Ok(match op.op_type {
            OpType::Add => lhs.add(&rhs),
            OpType::Subtract => lhs.subtract(&rhs),
            OpType::Multiply => lhs.multiply(&rhs),
            OpType::Divide => lhs.divide(&rhs),
            OpType::Equal => lhs.equal(&rhs),
            OpType::GreaterThan => lhs.greater_than(&rhs),
            OpType::LessThan => lhs.less_than(&rhs),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Position;

    #[test]
    fn literal() {
        let mut run = Runtime::new(&Scope::new());

        let value = Value::Number(42.0);
        let exp = Exp::new_literal(value.clone(), Position::new("test"));
        assert_eq!(run.run(&exp).unwrap(), value);
    }

    #[test]
    fn block() {
        let mut run = Runtime::new(&Scope::new());

        let exp = Exp::new_block(
            vec![Exp::new_literal(Value::Number(42.0), Position::new("test"))],
            Position::new("test"),
        );
        assert_eq!(run.run(&exp).unwrap(), Value::Nothing);
    }

    #[test]
    fn call_not_found() {
        let mut run = Runtime::new(&Scope::new());

        let exp = Exp::new_call("test".to_string(), Vec::new(), Position::new("test"));
        run.run(&exp).unwrap_err();
    }

    #[test]
    fn set_get_variable() {
        let mut run = Runtime::new(&Scope::new());
        let set = Exp::new_assignment(
            "test".to_string(),
            Box::new(Exp::new_literal(Value::Number(42.0), Position::new("test"))),
            Position::new("test"),
        );
        run.run(&set).unwrap();
        let get = Exp::new_variable("test".to_string(), Position::new("test"));
        assert_eq!(run.run(&get).unwrap(), Value::Number(42.0));
    }
}
