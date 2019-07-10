use std::{cmp, fmt, sync::Arc};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nothing,
    Int(i64),
    Function(Arc<&'static OmgFn>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
            Value::Function(_) => "BuiltIn function".to_string(),
        }
    }
}

pub trait OmgFnTr: Fn(&[Value]) -> Value {}
impl<F> OmgFnTr for F where F: Fn(&[Value]) -> Value + Copy {}
pub type OmgFn = dyn OmgFnTr<Output = Value>;

#[cfg_attr(tarpaulin, skip)]
impl fmt::Debug for OmgFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OmgFn")
    }
}

#[cfg_attr(tarpaulin, skip)]
impl cmp::PartialEq for OmgFn {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(tarpaulin, skip)]
    fn noop(_: &[Value]) -> Value {
        Value::Nothing
    }

    #[test]
    fn nothing_to_string() {
        assert_eq!(Value::Nothing.to_string(), "Nothing")
    }

    #[test]
    fn int_to_string() {
        assert_eq!(Value::Int(42).to_string(), "42")
    }

    #[test]
    fn function_to_string() {
        assert_eq!(
            Value::Function(Arc::new(&noop)).to_string(),
            "BuiltIn function"
        )
    }
}
