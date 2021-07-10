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
                Literal::Float(right_float) => Literal::Float((left_int as f64) + right_float),
                _ => Literal::None,
            },
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float + right_float),
                Literal::Int(right_int) => Literal::Float(left_float + (right_int as f64)),
                _ => Literal::None,
            },
            Literal::String(left_str) => match right_literal {
                Literal::String(right_str) => Literal::String(format!("{}{}", left_str, right_str)),
                _ => Literal::None,
            },
            _ => return Err("Can't add booleans".to_string()),
        })
    }

    pub fn sub(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int - right_int),
                Literal::Float(right_float) => Literal::Float((left_int as f64) - right_float),
                _ => return Err("Substraction work only with numbers".to_string()),
            },
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float - right_float),
                Literal::Int(right_int) => Literal::Float(left_float - (right_int as f64)),
                _ => return Err("Substraction work only with numbers".to_string()),
            },
            _ => return Err("Substraction work only with numbers".to_string()),
        })
    }

    pub fn mul(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int * right_int),
                Literal::Float(right_float) => Literal::Float((left_int as f64) * right_float),
                _ => return Err("Multiplication work only with numbers".to_string()),
            },
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float * right_float),
                Literal::Int(right_int) => Literal::Float(left_float * (right_int as f64)),
                _ => return Err("Multiplication work only with numbers".to_string()),
            },
            _ => return Err("Multiplication work only with numbers".to_string()),
        })
    }

    pub fn div(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int / right_int),
                Literal::Float(right_float) => Literal::Float((left_int as f64) / right_float),
                _ => return Err("Divisions work only with numbers".to_string()),
            },
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float / right_float),
                Literal::Int(right_int) => Literal::Float(left_float / (right_int as f64)),
                _ => return Err("Divisions work only with numbers".to_string()),
            },
            _ => return Err("Divisions work only with numbers".to_string()),
        })
    }

    pub fn r#mod(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match left_literal {
            Literal::Int(left_int) => match right_literal {
                Literal::Int(right_int) => Literal::Int(left_int % right_int),
                Literal::Float(right_float) => Literal::Float((left_int as f64) % right_float),
                _ => return Err("Mods work only with numbers".to_string()),
            },
            Literal::Float(left_float) => match right_literal {
                Literal::Float(right_float) => Literal::Float(left_float % right_float),
                Literal::Int(right_int) => Literal::Float(left_float % (right_int as f64)),
                _ => return Err("Mods work only with numbers".to_string()),
            },
            _ => return Err("Mods work only with numbers".to_string()),
        })
    }
}
