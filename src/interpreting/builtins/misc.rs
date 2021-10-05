use crate::{
    interpreting::{interpreter::Interpreter, typing::Literal},
    parsing::typing::Expr,
};

impl Interpreter {
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
}
