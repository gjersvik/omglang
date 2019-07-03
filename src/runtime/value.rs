use std::{cmp, fmt, sync::Arc};

#[derive(Debug, PartialEq)]
pub enum Value {
    Nothing,
    UInt(u64),
    Function(Box<Arc<OmgFn>>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::UInt(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
            Value::Function(_) => "BuiltIn function".to_string(),
        }
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
