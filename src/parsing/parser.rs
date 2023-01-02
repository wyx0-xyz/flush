use super::typing::*;
use crate::error::{FlushError, Result};
use crate::lexing::typing::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    file_path: PathBuf,
    statements: Vec<Statement>,
    position: usize,
}

#[allow(unreachable_patterns)]
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, file_path: PathBuf) -> Self {
        Self {
            tokens,
            file_path,
            statements: vec![],
            position: 0,
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.position - 1].clone()
    }

    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }

        self.position += 1;
        Some(self.previous().clone())
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        if self.is_at_end() {
            return Err(FlushError(
                self.file_path.clone(),
                self.previous().line,
                format!("Expected {:?}, found nothing", expected),
            ));
        }

        self.position += 1;

        let next = self.previous().clone();

        if next.kind != expected {
            return Err(FlushError(
                self.file_path.clone(),
                next.line,
                format!("Expected {:?}, found {:?}", expected, next.kind),
            ));
        }

        Ok(next)
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() > 0 && self.position == self.tokens.len()
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        Ok(match self.advance().unwrap().kind {
            TokenKind::If => self.parse_control_flow()?,
            TokenKind::Def => self.parse_def_set(true)?,
            TokenKind::Set => self.parse_def_set(false)?,
            TokenKind::Return => Statement::Return(self.parse_expr()?),
            TokenKind::While => self.parse_while()?,
            TokenKind::For => self.parse_for()?,
            TokenKind::Break => Statement::Break,
            TokenKind::Load => self.parse_load()?,
            unknow => {
                self.position -= 1;
                match self.parse_expr() {
                    Ok(expr) => Statement::Expr(expr),
                    _ => {
                        return Err(FlushError(
                            self.file_path.clone(),
                            self.previous().line,
                            format!("Unknow statement {:?}", unknow),
                        ));
                    }
                }
            }
        })
    }

    fn parse_control_flow(&mut self) -> Result<Statement> {
        self.expect(TokenKind::LParen)?;

        let condition = self.parse_expr()?;

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let mut if_body: Vec<Box<Statement>> = vec![];
        let mut else_body: Vec<Box<Statement>> = vec![];

        while !self.is_at_end() && self.current().kind != TokenKind::RBrace {
            if_body.push(Box::new(self.parse_statement()?));
        }

        self.expect(TokenKind::RBrace)?;

        if self.is_at_end() {
            return Ok(Statement::If(condition, if_body, vec![]));
        }

        if self.advance().unwrap().kind == TokenKind::Else {
            self.expect(TokenKind::LBrace)?;

            while !self.is_at_end() && self.current().kind != TokenKind::RBrace {
                else_body.push(Box::new(self.parse_statement()?));
            }

            self.expect(TokenKind::RBrace)?;
        } else {
            self.position -= 1;
        }

        Ok(Statement::If(condition, if_body, else_body))
    }

    fn parse_def_set(&mut self, def: bool) -> Result<Statement> {
        let id = match self.advance() {
            Some(token) => match token.kind {
                TokenKind::Ident(id) => id,
                kind => {
                    return Err(FlushError(
                        self.file_path.clone(),
                        token.line,
                        format!("Expected Identifier, found {:?}", kind),
                    ))
                }
            },
            _ => {
                return Err(FlushError(
                    self.file_path.clone(),
                    self.previous().line,
                    "Expected Identifier, found nothing".to_string(),
                ))
            }
        };

        let token = match self.advance() {
            Some(token) => token,
            None => {
                return Err(FlushError(
                    self.file_path.clone(),
                    self.previous().line,
                    "Expected Equal, found nothing".to_string(),
                ))
            }
        };

        Ok(match token.kind {
            TokenKind::Assign => (if def {
                Statement::VarDef
            } else {
                Statement::VarSet
            })(id, self.parse_expr()?),
            TokenKind::LParen => self.parse_func_def(id)?,
            unexpected => {
                return Err(FlushError(
                    self.file_path.clone(),
                    token.line,
                    format!("Unexpected token {:?}", unexpected),
                ))
            }
        })
    }

    fn parse_func_def(&mut self, id: String) -> Result<Statement> {
        let mut args: Vec<String> = vec![];

        while !self.is_at_end() && self.current().kind != TokenKind::RParen {
            match self.advance().unwrap().kind {
                TokenKind::Ident(id) => args.push(id),
                unexpected => {
                    return Err(FlushError(
                        self.file_path.clone(),
                        self.previous().line,
                        format!("Unexpected token {:?}", unexpected),
                    ))
                }
            };

            if self.current().kind == TokenKind::RParen {
                break;
            }

            self.expect(TokenKind::Comma)?;
        }

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let mut body: Vec<Statement> = vec![];

        while !self.is_at_end() {
            let current = match self.advance() {
                Some(token) => token,
                None => {
                    return Err(FlushError(
                        self.file_path.clone(),
                        self.previous().line,
                        "Unfinished function body".to_string(),
                    ))
                }
            };

            self.position -= 1;

            if current.kind == TokenKind::RBrace {
                break;
            }

            body.push(self.parse_statement()?);
        }

        self.expect(TokenKind::RBrace)?;

        Ok(Statement::FuncDef(id, args, body))
    }

    fn parse_while(&mut self) -> Result<Statement> {
        self.expect(TokenKind::LParen)?;

        let condition = self.parse_expr()?;

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let mut body: Vec<Box<Statement>> = vec![];

        while !self.is_at_end() && self.current().kind != TokenKind::RBrace {
            body.push(Box::new(self.parse_statement()?));
        }

        self.expect(TokenKind::RBrace)?;

        Ok(Statement::While(condition, body))
    }

    fn parse_for(&mut self) -> Result<Statement> {
        self.expect(TokenKind::LParen)?;

        let id = match self.parse_expr()? {
            Expr::Var(id) => id,
            unexpected => {
                return Err(FlushError(
                    self.file_path.clone(),
                    self.previous().line,
                    format!("Expected Identifier, found {:?}", unexpected),
                ))
            }
        };

        self.expect(TokenKind::In)?;

        let iterator = self.parse_expr()?;

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let mut body: Vec<Box<Statement>> = vec![];

        while !self.is_at_end() && self.current().kind != TokenKind::RBrace {
            body.push(Box::new(self.parse_statement()?));
        }

        self.expect(TokenKind::RBrace)?;

        Ok(Statement::For(id, iterator, body))
    }

    fn parse_load(&mut self) -> Result<Statement> {
        let raw_file_path = match self.parse_expr()? {
            Expr::String(path) => path,
            unexpected => {
                return Err(FlushError(
                    self.file_path.clone(),
                    self.previous().line,
                    format!("Expected String, found {:?}", unexpected),
                ))
            }
        };

        Ok(Statement::Load(raw_file_path))
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let next = match self.advance() {
            Some(token) => token,
            unexpected => {
                return Err(FlushError(
                    self.file_path.clone(),
                    self.previous().line,
                    format!("Expected Expression, found {:?}", unexpected),
                ))
            }
        };

        let expr = match next.kind {
            TokenKind::String(string) => Expr::String(string),
            TokenKind::Int(int) => Expr::Int(int),
            TokenKind::Float(float) => Expr::Float(float),
            TokenKind::Boolean(boolean) => Expr::Boolean(boolean),
            TokenKind::Ident(id) => {
                if self.is_at_end() {
                    return Ok(Expr::Var(id));
                }

                match self.advance().unwrap().kind {
                    TokenKind::LParen => self.parse_func_call(id)?,
                    TokenKind::LBracket => self.parse_list_index(Expr::Var(id))?,
                    _ => {
                        self.position -= 1;
                        Expr::Var(id)
                    }
                }
            }
            TokenKind::LBracket => self.parse_list()?,
            TokenKind::LBrace => self.parse_dict()?,
            TokenKind::Op(op) => {
                if self.is_at_end() {
                    return Err(FlushError(
                        self.file_path.clone(),
                        self.previous().line,
                        "Expected Number, found nothing".to_string(),
                    ));
                }

                let expr = self.parse_number()?;

                match expr {
                    Expr::Int(int) => {
                        if Op::Sub == op {
                            return Ok(Expr::Int(-int));
                        }
                        return Err(FlushError(
                            self.file_path.clone(),
                            self.previous().line,
                            format!("Expected Expression, found {:?}", op),
                        ));
                    }
                    Expr::Float(float) => {
                        if Op::Sub == op {
                            return Ok(Expr::Float(-float));
                        }

                        return Err(FlushError(
                            self.file_path.clone(),
                            self.previous().line,
                            format!("Expected Expression, found {:?}", op),
                        ));
                    }
                    _ => unreachable!(),
                }
            }
            unexpected => {
                return Err(FlushError(
                    self.file_path.clone(),
                    next.line,
                    format!("Expected Expression, found {:?}", unexpected),
                ))
            }
        };

        if self.is_at_end() {
            return Ok(expr);
        }

        Ok(match self.advance().unwrap().kind {
            TokenKind::Op(op) => self.parse_bin_op(expr, op)?,
            _ => {
                self.position -= 1;
                return Ok(expr);
            }
        })
    }

    fn parse_list(&mut self) -> Result<Expr> {
        let mut expressions: Vec<Box<Expr>> = vec![];

        while !self.is_at_end() && self.current().kind != TokenKind::RBracket {
            expressions.push(Box::new(self.parse_expr()?));

            if self.current().kind == TokenKind::RBracket {
                break;
            }

            self.expect(TokenKind::Comma)?;
        }

        self.expect(TokenKind::RBracket)?;

        if !self.is_at_end() {
            let next = self.advance().unwrap();

            if next.kind == TokenKind::LBracket {
                return Ok(self.parse_list_index(Expr::List(expressions))?);
            }

            self.position -= 1;
        }

        Ok(Expr::List(expressions))
    }

    fn parse_list_index(&mut self, expr: Expr) -> Result<Expr> {
        let index = self.parse_expr()?;

        self.expect(TokenKind::RBracket)?;

        Ok(Expr::ListIndex(Box::new(expr), Box::new(index)))
    }

    fn parse_dict(&mut self) -> Result<Expr> {
        let mut dict: HashMap<String, Box<Expr>> = HashMap::new();

        while !self.is_at_end() && self.current().kind != TokenKind::RBrace {
            let key = match self.parse_expr()? {
                Expr::String(key) => key,
                unexpected => {
                    return Err(FlushError(
                        self.file_path.clone(),
                        self.previous().line,
                        format!("Expected String, found {:?}", unexpected),
                    ))
                }
            };

            self.expect(TokenKind::Colon)?;

            dict.insert(key, Box::from(self.parse_expr()?));

            if self.current().kind == TokenKind::RBrace {
                break;
            }

            self.expect(TokenKind::Comma)?;
        }

        self.advance();

        Ok(Expr::Dictionnary(dict))
    }

    fn parse_number(&mut self) -> Result<Expr> {
        Ok(match self.parse_expr()? {
            Expr::Int(int) => Expr::Int(int),
            Expr::Float(float) => Expr::Float(float),
            unexpected => {
                return Err(FlushError(
                    self.file_path.clone(),
                    self.previous().line,
                    format!("Expected Number, found {:?}", unexpected),
                ))
            }
        })
    }

    fn parse_bin_op(&mut self, expr: Expr, bin_op: Op) -> Result<Expr> {
        let boxed_expr = Box::new(expr);
        let parsed_expr = Box::new(self.parse_expr()?);

        Ok(Expr::BinOp(
            match bin_op {
                Op::Add => BinOp::Add,
                Op::Sub => BinOp::Sub,
                Op::Mul => BinOp::Mul,
                Op::Div => BinOp::Div,
                Op::FloorDiv => BinOp::FloorDiv,
                Op::Mod => BinOp::Mod,
                Op::Pow => BinOp::Pow,
                Op::Lt => BinOp::Lt,
                Op::Gt => BinOp::Gt,
                Op::Eq => BinOp::Eq,
                Op::Ne => BinOp::Ne,
                Op::Le => BinOp::Le,
                Op::Ge => BinOp::Ge,
                _ => unreachable!(),
            },
            boxed_expr,
            parsed_expr,
        ))
    }

    fn parse_func_call(&mut self, id: String) -> Result<Expr> {
        let mut args: Vec<Box<Expr>> = vec![];

        while !self.is_at_end() && self.current().kind != TokenKind::RParen {
            args.push(Box::new(self.parse_expr()?));

            if self.current().kind == TokenKind::RParen {
                break;
            }

            self.expect(TokenKind::Comma)?;
        }

        self.expect(TokenKind::RParen)?;

        if !self.is_at_end() {
            if self.advance().unwrap().kind == TokenKind::LBracket {
                return Ok(self.parse_list_index(Expr::Call(id, args))?);
            }

            self.position -= 1;
        }

        Ok(Expr::Call(id, args))
    }

    pub fn parse(&mut self) -> Result<&Vec<Statement>> {
        while !self.is_at_end() {
            let statement = self.parse_statement()?;
            self.statements.push(statement.clone());
        }

        Ok(&self.statements)
    }
}
