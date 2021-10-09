use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::typing::Literal;
use crate::parsing::typing::Expr;

impl<'a> Interpreter<'a> {
    pub fn add(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match (left_literal, right_literal) {
            (Literal::Int(left), Literal::Int(right)) => Literal::Int(left + right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left + right),
            (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) + right),
            (Literal::String(left), Literal::String(right)) => {
                Literal::String(format!("{}{}", left, right))
            }
            (Literal::Float(left), Literal::Int(right)) => Literal::Float(left + (right as f64)),
            (left, right) => return Err(format!("Can't add {} and {}", left, right)),
        })
    }

    pub fn sub(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match (left_literal, right_literal) {
            (Literal::Int(left), Literal::Int(right)) => Literal::Int(left - right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left - right),
            (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) - right),
            (Literal::Float(left), Literal::Int(right)) => Literal::Float(left - (right as f64)),
            (left, right) => return Err(format!("Can't substract {} and {}", left, right)),
        })
    }

    pub fn mul(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match (left_literal, right_literal) {
            (Literal::Int(left), Literal::Int(right)) => Literal::Int(left * right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left * right),
            (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) * right),
            (Literal::Float(left), Literal::Int(right)) => Literal::Float(left * (right as f64)),
            (Literal::Int(left), Literal::String(right)) => {
                Literal::String(right.repeat(left as usize))
            }
            (Literal::String(left), Literal::Int(right)) => {
                Literal::String(left.repeat(right as usize))
            }
            (left, right) => return Err(format!("Can't multiply {} and {}", left, right)),
        })
    }

    pub fn div(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match (left_literal, right_literal) {
            (Literal::Int(left), Literal::Int(right)) => Literal::Int(left / right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left / right),
            (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) / right),
            (Literal::Float(left), Literal::Int(right)) => Literal::Float(left / (right as f64)),
            (left, right) => return Err(format!("Can't divide {} and {}", left, right)),
        })
    }

    pub fn r#mod(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match (left_literal, right_literal) {
            (Literal::Int(left), Literal::Int(right)) => Literal::Int(left % right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left % right),
            (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) % right),
            (Literal::Float(left), Literal::Int(right)) => Literal::Float(left % (right as f64)),
            (left, right) => return Err(format!("Can't mod {} and {}", left, right)),
        })
    }

    pub fn pow(&mut self, left: Box<Expr>, right: Box<Expr>) -> Result<Literal, String> {
        let left_literal = self.get_literal(*left)?;
        let right_literal = self.get_literal(*right)?;

        Ok(match (left_literal, right_literal) {
            (Literal::Int(left), Literal::Int(right)) => Literal::Int(left.pow(right as u32)),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left.powf(right)),
            (Literal::Int(left), Literal::Float(right)) => {
                Literal::Float((left as f64).powf(right))
            }
            (Literal::Float(left), Literal::Int(right)) => Literal::Float(left.powf(right as f64)),
            _ => return Err("Pow functions works only with numbers".to_string()),
        })
    }

    pub fn cos(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() <= 0 {
            return Err("Cosine function needs one argument".to_string());
        }

        let literal = self.get_literal(*args[0].clone())?;

        Ok(match literal {
            Literal::Int(int) => Literal::Float((int as f64).cos()),
            Literal::Float(float) => Literal::Float(float.cos()),
            _ => return Err("Cosine works only with numbers".to_string()),
        })
    }

    pub fn sin(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() <= 0 {
            return Err("Sine function needs one argument".to_string());
        }

        let literal = self.get_literal(*args[0].clone())?;

        Ok(match literal {
            Literal::Int(int) => Literal::Float((int as f64).sin()),
            Literal::Float(float) => Literal::Float(float.sin()),
            _ => return Err("Sine works only with numbers".to_string()),
        })
    }

    pub fn tan(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() <= 0 {
            return Err("Tangent function needs one argument".to_string());
        }

        let literal = self.get_literal(*args[0].clone())?;

        Ok(match literal {
            Literal::Int(int) => Literal::Float((int as f64).cos()),
            Literal::Float(float) => Literal::Float(float.cos()),
            _ => return Err("Tangent works only with numbers".to_string()),
        })
    }

    pub fn acos(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() <= 0 {
            return Err("ArcCosine function needs one argument".to_string());
        }

        let literal = self.get_literal(*args[0].clone())?;

        Ok(match literal {
            Literal::Int(int) => Literal::Float((int as f64).acos()),
            Literal::Float(float) => Literal::Float(float.acos()),
            _ => return Err("ArcCosine works only with numbers".to_string()),
        })
    }

    pub fn asin(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() <= 0 {
            return Err("ArcSine function needs one argument".to_string());
        }

        let literal = self.get_literal(*args[0].clone())?;

        Ok(match literal {
            Literal::Int(int) => Literal::Float((int as f64).asin()),
            Literal::Float(float) => Literal::Float(float.asin()),
            _ => return Err("ArcSine works only with numbers".to_string()),
        })
    }

    pub fn atan(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() <= 0 {
            return Err("ArcTangent function needs one argument".to_string());
        }

        let literal = self.get_literal(*args[0].clone())?;

        Ok(match literal {
            Literal::Int(int) => Literal::Float((int as f64).acos()),
            Literal::Float(float) => Literal::Float(float.acos()),
            _ => return Err("ArcTangent works only with numbers".to_string()),
        })
    }
}
