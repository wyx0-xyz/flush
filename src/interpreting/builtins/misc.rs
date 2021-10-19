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
}
