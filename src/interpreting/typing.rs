use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Int(i32),
    Float(f32),
    Boolean(bool),
    None
}

#[derive(PartialEq)]
pub enum ScopeContext {
    Definition,
    TopLevel
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(string) => write!(f, "{}", string),
            Value::Int(int) => write!(f, "{}", int),
            Value::Float(float) => write!(f, "{}", float),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::None => write!(f, "None"),
        }
    }
}
