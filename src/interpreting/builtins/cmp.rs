use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::typing::Literal;
use crate::parsing::typing::Expr;
use std::cmp::Ordering;

impl PartialOrd for Literal {
    fn partial_cmp(&self, right: &Self) -> Option<Ordering> {
        Some(match self {
            Literal::String(left_str) => match right {
                Literal::String(right_str) => left_str.len().cmp(&right_str.len()),
                _ => return None
            }
            Literal::Int(left_int) => match right {
                Literal::Int(right_int) => left_int.cmp(&right_int),
                Literal::Float(right_float) => (*left_int as f64).partial_cmp(&right_float).unwrap(),
                _ => return None
            }
            Literal::Float(left_float) => match right {
                Literal::Float(right_float) => left_float.partial_cmp(&right_float).unwrap(),
                Literal::Int(right_int) => left_float.partial_cmp(&(*right_int as f64)).unwrap(),
                _ => return None
            }
            _ => return None
        })
    }
}

impl PartialEq for Literal {
    fn eq(&self, right: &Self) -> bool {
        match self {
            Literal::String(left_str) => match right {
                Literal::String(right_str) => left_str == right_str,
                _ => false,
            },
            Literal::Int(left_int) => match right {
                Literal::Int(right_int) => left_int == right_int,
                Literal::Float(right_float) => (*left_int as f64) == *right_float,
                _ => false,
            },
            Literal::Float(left_float) => match right {
                Literal::Float(right_float) => left_float == right_float,
                Literal::Int(right_int) => *left_float == (*right_int as f64),
                _ => false,
            },
            Literal::Boolean(left_bool) => match right {
                Literal::Boolean(right_bool) => left_bool == right_bool,
                _ => false,
            },
            _ => false,
        }
    }

    fn ne(&self, right: &Self) -> bool {
        match self {
            Literal::String(left_str) => match right {
                Literal::String(right_str) => left_str != right_str,
                _ => false,
            },
            Literal::Int(left_int) => match right {
                Literal::Int(right_int) => left_int != right_int,
                Literal::Float(right_float) => (*left_int as f64) == *right_float,
                _ => false,
            },
            Literal::Float(left_float) => match right {
                Literal::Float(right_float) => left_float != right_float,
                Literal::Int(right_int) => *left_float != (*right_int as f64),
                _ => false,
            },
            Literal::Boolean(left_bool) => match right {
                Literal::Boolean(right_bool) => left_bool != right_bool,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Interpreter {
    pub fn lt(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(Literal::Boolean(left_literal < right_literal))
    }
    
    pub fn gt(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(Literal::Boolean(left_literal > right_literal))
    }

    pub fn eq(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(Literal::Boolean(left_literal == right_literal))
    }

    pub fn ne(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(Literal::Boolean(left_literal != right_literal))
    }

    pub fn le(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(Literal::Boolean(left_literal <= right_literal))
    }

    pub fn ge(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(Literal::Boolean(left_literal >= right_literal))
    }
}
