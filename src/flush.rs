use std::collections::HashMap;
use std::fs::{canonicalize, read_to_string};
use std::path::PathBuf;

use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::typing::Literal;
use crate::lexing::lexer::Lexer;
use crate::parsing::parser::Parser;

pub fn process_file_path(raw_file_path: &str) -> Result<PathBuf, String> {
    let file_path = PathBuf::from(raw_file_path);

    if !file_path.exists() {
        return Err("Path does not exist!".to_string());
    }

    if !file_path.is_file() {
        return Err("Path isn't a file!".to_string());
    }

    let absolute_path = canonicalize(file_path).unwrap();

    Ok(absolute_path)
}

pub fn run(
    raw_file_path: &str,
    cache: &mut HashMap<PathBuf, HashMap<String, Literal>>,
) -> Result<(), String> {
    let file_path = process_file_path(raw_file_path)?;

    if cache.contains_key(&file_path) {
        return Ok(());
    }

    let file_content = match read_to_string(file_path.clone()) {
        Ok(content) => content,
        Err(e) => return Err(format!("Couldn't open file: {}", e)),
    };

    let mut lexer = Lexer::new(&file_content, file_path.clone());
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => return Err(e.to_string()),
    };

    let mut parser = Parser::new(tokens, file_path.clone());
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => return Err(e.to_string()),
    };

    let mut interpreter = Interpreter::new(statements.clone());

    interpreter.interpret()?;

    let stack = interpreter.get_stack();

    cache.insert(file_path, stack);

    Ok(())
}
