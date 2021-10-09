mod error;
mod flush;
mod interpreting;
mod lexing;
mod parsing;

use crate::interpreting::typing::Literal;
use clap::{App, Arg};
use std::{collections::HashMap, path::PathBuf};

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
    let mut cache: HashMap<PathBuf, HashMap<String, Literal>> = HashMap::new();

    match flush::run(raw_file_path, &mut cache) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
