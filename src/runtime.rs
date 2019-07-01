mod scope;

use crate::parser::Exp;

pub enum Value {
    Nothing,
    UInt(u64),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::UInt(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
        }
    }
}

pub fn run_exp(exp: &Exp) -> Value {
    match exp {
        Exp::Call(i, args) => {
            if i != "print" {
                panic!("Cant find function named {} to call", i);
            }
            for exp in args {
                println!("{}", run_exp(&exp).to_string());
            }
            Value::Nothing
        }
        Exp::Block(block) => {
            for e in block {
                run_exp(e);
            }
            Value::Nothing
        }
        Exp::LiteralUInt(int) => Value::UInt(*int),
        Exp::InValid => panic!("Invalid expression found."),
    }
}
