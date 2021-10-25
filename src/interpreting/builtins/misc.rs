use crate::{
    interpreting::{interpreter::Interpreter, typing::Literal},
    parsing::typing::Expr,
};
use rand::Rng;

impl<'a> Interpreter<'a> {
    pub fn range(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() < 2 {
            return Err(format!("Expected 2 arguments, but given {}.", args.len()));
        }

        match (
            self.get_literal(*args[0].clone())?,
            self.get_literal(*args[1].clone())?,
        ) {
            (Literal::Int(start), Literal::Int(stop)) => Ok(Literal::List(
                (start..stop)
                    .into_iter()
                    .map(|i| Box::new(Literal::Int(i)))
                    .collect(),
            )),
            _ => Err(format!("Range start and stop must be Integers.")),
        }
    }

    pub fn random(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() < 2 {
            return Err(format!(
                "Expected 2 arguments: min and max, but given {}.",
                args.len()
            ));
        }

        let mut rng = rand::thread_rng();

        match (
            self.get_literal(*args[0].clone())?,
            self.get_literal(*args[1].clone())?,
        ) {
            (Literal::Int(min), Literal::Int(max)) => Ok(Literal::Int(rng.gen_range(min..max))),
            (Literal::Float(min), Literal::Float(max)) => {
                Ok(Literal::Float(rng.gen_range(min..max)))
            }
            (Literal::Int(min), Literal::Float(max)) => {
                Ok(Literal::Float(rng.gen_range((min as f64)..max)))
            }
            (Literal::Float(min), Literal::Int(max)) => {
                Ok(Literal::Float(rng.gen_range(min..(max as f64))))
            }
            _ => Err(format!("Random min and max must be Numbers.")),
        }
    }

    pub fn parse_int(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() < 1 {
            return Err(format!("Expected 1 argument but given {}", args.len()));
        }

        let raw_int = match self.get_literal(*args[0].clone())? {
            Literal::String(string) => string,
            unexpected => return Err(format!("Expected String found '{}'", unexpected)),
        };

        Ok(match raw_int.parse::<i32>() {
            Ok(int) => Literal::Int(int),
            Err(_) => return Err(format!("Invalid Integer '{}'", raw_int)),
        })
    }

    pub fn parse_float(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() < 1 {
            return Err(format!("Expected 1 argument but given {}", args.len()));
        }

        let raw_float = match self.get_literal(*args[0].clone())? {
            Literal::String(string) => string,
            unexpected => return Err(format!("Expected String found '{}'", unexpected)),
        };

        Ok(match raw_float.parse::<f64>() {
            Ok(float) => Literal::Float(float),
            Err(_) => return Err(format!("Invalid Float '{}'", raw_float)),
        })
    }

    pub fn to_string(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if args.len() < 1 {
            return Err(format!("Expected 1 argument but given {}", args.len()));
        }

        Ok(Literal::String(
            self.get_literal(*args[0].clone())?.to_string(),
        ))
    }
}
