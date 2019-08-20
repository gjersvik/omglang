use crate::function::Function;
use crate::module_scope::ModuleScope;
use crate::value::Value;
use im::Vector;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Native {
    Print,
}

impl Native {
    pub fn call(self, args: Vector<Value>) -> Value {
        match self {
            Native::Print => print(args),
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
fn print(args: Vector<Value>) -> Value {
    let string = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", string);
    Value::Nothing
}

pub fn add_std_lib(module: &ModuleScope) -> ModuleScope {
    module.add_function("print", Function::NativeFunction(Native::Print))
}

#[cfg(test)]
mod tests {}
