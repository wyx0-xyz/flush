use super::typing::*;
use crate::error::{FlushError, Result};
use std::path::PathBuf;

#[derive(Default)]
pub struct Lexer<'a> {
    program: &'a str,
    file_path: PathBuf,
    tokens: Vec<Token>,
    position: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str, file_path: PathBuf) -> Self {
        Self {
            program,
            file_path,
            tokens: vec![],
            position: 0,
            line: 1,
        }
    }

    fn previous(&self) -> Option<char> {
        self.program.chars().nth(self.position - 1)
    }

    fn current(&self) -> Option<char> {
        self.program.chars().nth(self.position)
    }

    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.program.len()
    }

    fn push_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            line: self.line,
            kind,
        });
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.advance() != Some('\n') {
            self.position += 1;
        }

        self.line += 1;
    }

    fn parse_string(&mut self) -> Result<()> {
        let mut string = String::new();

        while !self.is_at_end() && self.current() != Some('"') {
            match self.current() {
                Some(character) => {
                    if character == '\n' {
                        return Err(FlushError(
                            self.file_path.clone(),
                            self.line,
                            "Illegal newline in a string".to_string(),
                        ));
                    }

                    string.push(character);
                }
                None => break,
            };

            self.position += 1;
        }

        if self.current() != Some('"') {
            return Err(FlushError(
                self.file_path.clone(),
                self.line,
                "Unterminated string".to_string(),
            ));
        }

        self.position += 1;
        self.push_token(TokenKind::String(string));

        Ok(())
    }

    fn parse_number(&mut self) {
        let mut raw_number = String::from(self.previous().unwrap());

        while !self.is_at_end() {
            let current = match self.current() {
                Some(character) => character,
                None => break,
            };

            if current == '.' || current.is_ascii_digit() {
                raw_number.push(current);
                self.position += 1;
            } else {
                break;
            }
        }

        match raw_number.parse::<i32>() {
            Ok(int) => self.push_token(TokenKind::Int(int)),
            Err(_) => self.push_token(TokenKind::Float(raw_number.parse::<f64>().unwrap())),
        }
    }

    fn parse_identifier(&mut self) {
        let mut identifier = String::from(self.previous().unwrap());

        while !self.is_at_end() {
            let current = match self.current() {
                Some(character) => character,
                None => break,
            };

            if current == '_' || current.is_ascii_alphanumeric() {
                identifier.push(current);
                self.position += 1;
            } else {
                break;
            }
        }

        let token = match identifier.as_str() {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "break" => TokenKind::Break,
            "load" => TokenKind::Load,
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            ident => TokenKind::Ident(ident.to_string()),
        };

        self.push_token(token);
    }

    fn parse_token(&mut self) -> Result<()> {
        if let Some(character) = self.advance() {
            match character {
                '(' => self.push_token(TokenKind::LParen),
                ')' => self.push_token(TokenKind::RParen),
                '{' => self.push_token(TokenKind::LBrace),
                '}' => self.push_token(TokenKind::RBrace),
                '[' => self.push_token(TokenKind::LBracket),
                ']' => self.push_token(TokenKind::RBracket),
                ',' => self.push_token(TokenKind::Comma),
                ':' => self.push_token(TokenKind::Colon),
                '+' => self.push_token(TokenKind::Op(Op::Add)),
                '-' => self.push_token(TokenKind::Op(Op::Sub)),
                '*' => self.push_token(TokenKind::Op(Op::Mul)),
                '/' => {
                    if self.current() == Some('=') {
                        self.push_token(TokenKind::Op(Op::Ne));
                        self.position += 1;
                    } else if self.current() == Some('/') {
                        self.push_token(TokenKind::Op(Op::FloorDiv));
                        self.position += 1;
                    } else {
                        self.push_token(TokenKind::Op(Op::Div))
                    }
                }
                '%' => self.push_token(TokenKind::Op(Op::Mod)),
                '^' => self.push_token(TokenKind::Op(Op::Pow)),
                '<' => {
                    if self.current() == Some('=') {
                        self.push_token(TokenKind::Op(Op::Le));
                        self.position += 1;
                    } else {
                        self.push_token(TokenKind::Op(Op::Lt));
                    }
                }
                '>' => {
                    if self.current() == Some('=') {
                        self.push_token(TokenKind::Op(Op::Ge));
                        self.position += 1;
                    } else {
                        self.push_token(TokenKind::Op(Op::Gt));
                    }
                }
                '=' => {
                    if self.current() == Some('=') {
                        self.push_token(TokenKind::Op(Op::Eq));
                        self.position += 1;
                    } else {
                        self.push_token(TokenKind::Assign);
                    }
                }
                '"' => self.parse_string()?,
                '#' => self.skip_comment(),
                '\n' => self.line += 1,
                _ if character.is_ascii_digit() => self.parse_number(),
                '_' | _ if character.is_ascii_alphanumeric() => self.parse_identifier(),
                _ => (),
            };
        }

        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end() {
            self.parse_token()?;
        }

        Ok(&self.tokens)
    }
}
