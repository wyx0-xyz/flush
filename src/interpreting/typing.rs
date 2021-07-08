use crate::parsing::typing::Statement;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Int(i32),
    Float(f64),
    Boolean(bool),
    Function(String, Vec<String>, Vec<Statement>),
    None
}

#[derive(Clone, PartialEq)]
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
            Value::Function(name, args, _) => write!(f, "<function:{}#{}>", name, args.join(", ")),
            Value::None => write!(f, "None"),
        }
    }
}
