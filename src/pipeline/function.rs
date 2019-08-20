use crate::core_lib::Native;

#[derive(Clone)]
pub enum Function { 
    NativeFunction(Native),
}