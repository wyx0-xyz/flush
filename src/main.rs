mod error;
mod interpreting;
mod lexing;
mod parsing;

use clap::{App, Arg};
use interpreting::interpreter::Interpreter;
use lexing::lexer::Lexer;
use parsing::parser::Parser;
use std::{fs, path};

fn main() {
    let matches = App::new("flush-lang")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("flush-lang programming language")
        .arg(
            Arg::with_name("file")
                .required(true)
                .help("File to be interpreted"),
        )
        .get_matches();

    let raw_file_path = matches.value_of("file").unwrap();
    let file_path = path::Path::new(raw_file_path);

    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            return eprintln!("Couldn't open {}: {}", file_path.display(), error);
        }
    };

    let mut lexer = Lexer::new(file_content, file_path.to_str().unwrap_or(""));
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => return eprintln!("{}", e),
    };

    let mut parser = Parser::new(tokens, file_path.to_str().unwrap_or(""));
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => return eprintln!("{}", e),
    };

    let mut interpreter = Interpreter::new(statements);
    match interpreter.interpret() {
        Ok(()) => (),
        Err(e) => return eprintln!("{}", e),
    };
}
