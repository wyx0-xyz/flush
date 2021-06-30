mod error;
mod lexing;
// mod parser;

use clap::{App, Arg};
use lexing::lexer::Lexer;
use std::{fs, path, process};

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
            println!("Couldn't open {}: {}", file_path.display(), error);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(file_content, file_path.to_str().unwrap_or(""));
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => return eprintln!("{}", e),
    };

    println!("Tokens: {:?}", tokens);
}
