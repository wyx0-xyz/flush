mod error;
mod flush;
mod interpreting;
mod lexing;
mod parsing;

use ansi_term::Color::Red;
use std::env::args;
use std::path::PathBuf;

fn main() {
    let raw_file_path = match args().nth(1) {
        Some(path) => path,
        None => return eprintln!("{}: Usage: flush <file_path>", Red.paint("[error]")),
    };

    let mut cache: Vec<PathBuf> = vec![];

    match flush::run(&raw_file_path, &mut cache) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
