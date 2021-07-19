use crate::parsing::typing::Statement;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Literal {
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

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(string) => write!(f, "\"{}\"", string),
            Literal::Int(int) => write!(f, "{}", int),
            Literal::Float(float) => write!(f, "{}", float),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::Function(name, args, _) => write!(f, "<function:{}#{}>", name, args.join(", ")),
            Literal::None => write!(f, "None"),
        }
    }
}
