use super::typing::{Expr, Statement};
use crate::error::{FlushError, Result};
use crate::lexing::typing::{Token, TokenKind};

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    file: String,
    statements: Vec<Statement>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file: impl ToString) -> Self {
        Self {
            tokens,
            file: file.to_string(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        Ok(self.statements.clone())
    }
}
