use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::typing::Literal;
use crate::parsing::typing::Expr;
use std::io::Write;

impl<'a> Interpreter<'a> {
    pub fn put_str(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        for (i, arg) in args.iter().enumerate() {
            match self.get_literal(*arg.clone())? {
                Literal::String(string) => {
                    if i == args.len() - 1 {
                        print!("{}", string);
                    } else {
                        print!("{} ", string);
                    }
                }
                unexpected => {
                    return Err(format!(
                        "The `putStr` function expected a String, not `{}`",
                        unexpected
                    ))
                }
            };
        }

        Ok(Literal::None)
    }

    pub fn put_str_ln(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        self.put_str(args)?;

        println!();

        Ok(Literal::None)
    }

    pub fn print(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        for (i, arg) in args.iter().enumerate() {
            if i == args.len() - 1 {
                print!("{}", self.get_literal(*arg.clone())?);
            } else {
                print!("{} ", self.get_literal(*arg.clone())?);
            }
        }

        Ok(Literal::None)
    }

    pub fn print_ln(&mut self, args: Vec<Box<Expr>>) -> Result<Literal, String> {
        self.print(args)?;

        println!();

        Ok(Literal::None)
    }

    pub fn flush_stdout(&mut self, _: Vec<Box<Expr>>) -> Result<Literal, String> {
        match std::io::stdout().flush() {
            Ok(_) => Ok(Literal::None),
            Err(e) => Err(format!("Could not flush stdout: {}", e)),
        }
    }

    pub fn read_line(&mut self, _: Vec<Box<Expr>>) -> Result<Literal, String> {
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => Ok(Literal::String(input.trim_end().to_string())),
            Err(e) => return Err(e.to_string()),
        }
    }
}
