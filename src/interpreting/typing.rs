use crate::parsing::typing::Statement;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Int(i32),
    Float(f64),
    Boolean(bool),
    List(Vec<Box<Literal>>),
    Dictionnary(HashMap<String, Box<Literal>>),
    Function(String, Vec<String>, Vec<Statement>),
    None,
}

#[derive(Clone, PartialEq)]
pub enum Context {
    Function,
    TopLevel,
    Loop,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(string) => write!(f, "\"{}\"", string),
            Literal::Int(int) => write!(f, "{}", int),
            Literal::Float(float) => write!(f, "{}", float),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::List(list) => {
                write!(f, "[")?;

                for (i, expr) in list.into_iter().enumerate() {
                    if i == list.len() - 1 {
                        write!(f, "{}", expr)?;
                    } else {
                        write!(f, "{}, ", expr)?;
                    }
                }

                write!(f, "]")
            }
            Literal::Dictionnary(dict) => {
                write!(f, "{{")?;

                for (i, key) in dict.keys().enumerate() {
                    if i == dict.keys().len() - 1 {
                        write!(f, "\"{}\": {}", key, dict.get(key).unwrap())?;
                    } else {
                        write!(f, "\"{}\": {}, ", key, dict.get(key).unwrap())?;
                    }
                }

                write!(f, "}}")
            }
            Literal::Function(name, args, _) => {
                write!(f, "<function:{}#{}>", name, args.join(", "))
            }
            Literal::None => write!(f, "None"),
        }
    }
}
