use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::typing::Literal;
use crate::parsing::typing::Expr;

impl Interpreter {
    pub fn add(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int + right_int),
                Literal::Float(_) => return Err("Can't add integers and floats".to_string()),
                _ => Literal::None
            }
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float + right_float),
                Literal::Int(_) => return Err("Can't add floats and integers".to_string()),
                _ => Literal::None
            }
            Literal::String(left_str) => match right_literal {
                Literal::String(right_str) => Literal::String(format!("{}{}", left_str, right_str)),
                _ => Literal::None
            }
            _ => Literal::None
        })
    }

    pub fn sub(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int - right_int),
                Literal::Float(right_float) => Literal::Float((left_int as f64) - right_float),
                _ => Literal::None
            }
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float - right_float),
                Literal::Int(right_int) => Literal::Float(left_float - (right_int as f64)),
                _ => Literal::None
            }
            _ => Literal::None
        })
    }

    pub fn mul(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int * right_int),
                Literal::Float(_) => return Err("Can't multiply integers and floats".to_string()),
                _ => Literal::None
            }
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float * right_float),
                Literal::Int(_) => return Err("Can't multiply floats and integers".to_string()),
                _ => Literal::None
            }
            _ => Literal::None
        })
    }

    pub fn div(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int / right_int),
                Literal::Float(_) => return Err("Can't divide int and float".to_string()),
                _ => Literal::None
            }
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float / right_float),
                Literal::Int(_) => return Err("Can't divide float and int".to_string()),
                _ => Literal::None
            }
            _ => Literal::None
        })
    }

    pub fn r#mod(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int % right_int),
                Literal::Float(_) => return Err("Can't mod int and float".to_string()),
                _ => Literal::None
            }
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float % right_float),
                Literal::Int(_) => return Err("Can't mod float and int".to_string()),
                _ => Literal::None
            }
            _ => Literal::None
        })
    }
}
