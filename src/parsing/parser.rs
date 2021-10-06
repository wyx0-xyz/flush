use super::typing::*;
use crate::error::{FlushError, Result};
use crate::lexing::typing::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    file_path: &'a str,
    statements: Vec<Statement>,
    position: usize,
}

#[allow(unreachable_patterns)]
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, file_path: &'a str) -> Self {
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
                self.file_path.to_string(),
                self.previous().line,
                format!("Expected {:?} found nothing", expected),
            ));
        }

        self.position += 1;

        let next = self.previous().clone();

        if next.kind != expected {
            return Err(FlushError(
                self.file_path.to_string(),
                next.line,
                format!("Unexpected token {:?}, expected {:?}", next.kind, expected),
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
            TokenKind::Def => self.parse_def()?,
            TokenKind::Return => Statement::Return(self.parse_expr()?),
            TokenKind::While => self.parse_while()?,
            TokenKind::For => self.parse_for()?,
            TokenKind::Break => Statement::Break,
            unknow => {
                self.position -= 1;
                match self.parse_expr() {
                    Ok(expr) => Statement::Expr(expr),
                    _ => {
                        return Err(FlushError(
                            self.file_path.to_string(),
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

    fn parse_def(&mut self) -> Result<Statement> {
        let id = match self.advance() {
            Some(token) => match token.kind {
                TokenKind::Ident(id) => id,
                kind => {
                    return Err(FlushError(
                        self.file_path.to_string(),
                        token.line,
                        format!("Expected identifier found '{:?}'", kind),
                    ))
                }
            },
            _ => {
                return Err(FlushError(
                    self.file_path.to_string(),
                    self.previous().line,
                    "Expected identifier".to_string(),
                ))
            }
        };

        let token = match self.advance() {
            Some(token) => token,
            None => {
                return Err(FlushError(
                    self.file_path.to_string(),
                    self.previous().line,
                    "Unexpected token def".to_string(),
                ))
            }
        };

        Ok(match token.kind {
            TokenKind::Assign => Statement::VarDef(id, self.parse_expr()?),
            TokenKind::LParen => self.parse_func_def(id)?,
            unexpected => {
                return Err(FlushError(
                    self.file_path.to_string(),
                    token.line,
                    format!("Unexpected token: {:?}", unexpected),
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
                        self.file_path.to_string(),
                        self.previous().line,
                        format!("Unexpected token '{:?}'", unexpected),
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
                        self.file_path.to_string(),
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
                    self.file_path.to_string(),
                    self.previous().line,
                    format!("Expected Identifier found '{:?}'", unexpected),
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

    fn parse_expr(&mut self) -> Result<Expr> {
        let next = match self.advance() {
            Some(token) => token,
            unexpected => {
                return Err(FlushError(
                    self.file_path.to_string(),
                    self.previous().line,
                    format!("Expected expression found '{:?}'", unexpected),
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
                    _ => {
                        self.position -= 1;
                        Expr::Var(id)
                    }
                }
            }
            TokenKind::LBracket => self.parse_list()?,
            unexpected => {
                return Err(FlushError(
                    self.file_path.to_string(),
                    next.line,
                    format!("Expected expression found '{:?}'", unexpected),
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

        Ok(Expr::List(expressions))
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
                Op::Mod => BinOp::Mod,
                Op::Square => BinOp::Square,
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

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::error::Result;
    use crate::lexing::lexer::Lexer;
    use crate::parsing::typing::*;

    #[test]
    fn control_flow() -> Result<()> {
        let mut lexer = Lexer::new(r#"if (true) {} else {}"#, "__test__.flush");
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        assert_eq!(
            parser.parse()?,
            &vec![Statement::If(Expr::Boolean(true), vec![], vec![])]
        );

        Ok(())
    }

    #[test]
    fn var_def() -> Result<()> {
        let mut lexer = Lexer::new(r#"def username = "wyxo""#, "__test__.flush");
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        assert_eq!(
            parser.parse()?,
            &vec![Statement::VarDef(
                "username".to_string(),
                Expr::String("wyxo".to_string())
            )]
        );

        Ok(())
    }

    #[test]
    fn func_def() -> Result<()> {
        let mut lexer = Lexer::new(r#"def add(a, b) { return a + b }"#, "__test__.flush");
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        assert_eq!(
            parser.parse()?,
            &vec![Statement::FuncDef(
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                vec![Statement::Return(Expr::BinOp(
                    BinOp::Add,
                    Box::new(Expr::Var("a".to_string())),
                    Box::new(Expr::Var("b".to_string()))
                ))]
            )]
        );

        Ok(())
    }

    #[test]
    fn unterminated_func_def() -> Result<()> {
        let mut lexer = Lexer::new("def f(x) { return x * x", "__test__.flush");
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        match parser.parse() {
            Ok(_) => panic!(),
            Err(e) => assert_eq!(e.2, "Expected RBrace found nothing".to_string()),
        };

        Ok(())
    }

    #[test]
    fn while_loop() -> Result<()> {
        let mut lexer = Lexer::new(r#"while (false) { break }"#, "__test__.flush");
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        assert_eq!(
            parser.parse()?,
            &vec![Statement::While(
                Expr::Boolean(false),
                vec![Box::new(Statement::Break)]
            )]
        );

        Ok(())
    }

    #[test]
    fn for_loop() -> Result<()> {
        let mut lexer = Lexer::new(r#"for (i in [1]) { break }"#, "__test__.flush");
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        assert_eq!(
            parser.parse()?,
            &vec![Statement::For(
                "i".to_string(),
                Expr::List(vec![Box::new(Expr::Int(1))]),
                vec![Box::new(Statement::Break)]
            )]
        );

        Ok(())
    }

    #[test]
    fn expressions() -> Result<()> {
        let mut lexer = Lexer::new(
            r#""Hello, Flush!" 54 3.14 false user add(1, true, 4.0) [1, user, sin(28)]"#,
            "__test__.flush",
        );
        let mut parser = Parser::new(lexer.tokenize()?, "__test__.flush");

        assert_eq!(
            parser.parse()?,
            &vec![
                Statement::Expr(Expr::String("Hello, Flush!".to_string()),),
                Statement::Expr(Expr::Int(54)),
                Statement::Expr(Expr::Float(3.14)),
                Statement::Expr(Expr::Boolean(false)),
                Statement::Expr(Expr::Var("user".to_string())),
                Statement::Expr(Expr::Call(
                    "add".to_string(),
                    vec![
                        Box::new(Expr::Int(1)),
                        Box::new(Expr::Boolean(true)),
                        Box::new(Expr::Float(4.))
                    ]
                )),
                Statement::Expr(Expr::List(vec![
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Var("user".to_string())),
                    Box::new(Expr::Call("sin".to_string(), vec![Box::new(Expr::Int(28))]))
                ]))
            ]
        );

        Ok(())
    }
}
