use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::typing::Literal;
use crate::lexing::lexer::Lexer;
use crate::parsing::parser::Parser;
use crate::parsing::typing::Statement;
use ansi_term::Colour::{Blue, Red, Yellow};
use std::collections::HashMap;
use std::fs::{canonicalize, read_to_string};
use std::path::PathBuf;

pub fn process_file_path(raw_file_path: &str) -> Result<PathBuf, String> {
    let file_path = PathBuf::from(raw_file_path);

    if !file_path.exists() {
        return Err(format!("{}: Path does not exist!", Red.paint("[error]")));
    }

    if !file_path.is_file() {
        return Err(format!("{}: Path isn't a file!", Red.paint("[error]")));
    }

    let absolute_path = canonicalize(file_path).unwrap();

    Ok(absolute_path)
}

pub fn run(
    raw_file_path: &str,
    cache: &mut Vec<PathBuf>,
) -> Result<Option<HashMap<String, Literal>>, String> {
    let file_path = process_file_path(raw_file_path)?;

    if cache.contains(&file_path) {
        return Ok(None);
    }

    let file_content = match read_to_string(file_path.clone()) {
        Ok(content) => content,
        Err(e) => {
            return Err(format!(
                "{}: Couldn't open file: {}",
                Red.paint("[error]"),
                e
            ))
        }
    };

    let mut lexer = Lexer::new(&file_content, file_path.clone());
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => return Err(format!("{}: {}", Red.paint("[error]"), e)),
    };

    let mut parser = Parser::new(tokens, file_path.clone());
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => return Err(format!("{}: {}", Red.paint("[error]"), e)),
    };

    cache.push(file_path.clone());

    let mut new_statements: Vec<Statement> = vec![];

    for statement in statements.clone() {
        if let Statement::Load(raw_path) = statement.clone() {
            let absolute_path = process_file_path(&raw_path)?;

            if absolute_path == file_path || cache.contains(&absolute_path) {
                println!(
                    "{}: Detected cycle import: {} and {}",
                    Yellow.paint("[warning]"),
                    Blue.paint(file_path.to_string_lossy()),
                    Blue.paint(absolute_path.to_string_lossy())
                );
                continue;
            }
        }

        new_statements.push(statement);
    }

    let mut interpreter =
        Interpreter::new(new_statements, file_path.clone(), cache, cache.len() == 1);

    match interpreter.interpret() {
        Ok(_) => {}
        Err(e) => return Err(format!("{}: {}", Red.paint("[error]"), e)),
    }

    let stack = interpreter.get_stack();

    Ok(Some(stack))
}
