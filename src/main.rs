mod error;
mod flush;
mod interpreting;
mod lexing;
mod parsing;

use ansi_term::Color::Red;
use std::env::args;

fn main() {
    let raw_file_path = match args().nth(1) {
        Some(path) => path,
        None => return eprintln!("{}: Usage: flush <file_path>", Red.paint("[error]")),
    };

    let mut cache = vec![];

    if let Err(e) = flush::run(&raw_file_path, &mut cache) {
        eprintln!("{}", e);
    }
}
