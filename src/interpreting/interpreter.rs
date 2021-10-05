use crate::interpreting::typing::*;
use crate::parsing::typing::*;
use std::collections::HashMap;

pub struct Interpreter {
    statements: Vec<Statement>,
    stack: Vec<HashMap<String, Literal>>,
    builtins: HashMap<String, fn(&mut Self, Vec<Box<Expr>>) -> Result<Literal, String>>,
    context: ScopeContext,
    position: usize,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Self {
        let mut interpreter = Self {
            statements,
            stack: vec![HashMap::new()], // TopLevel scope
            builtins: HashMap::new(),
            context: ScopeContext::TopLevel,
            position: 0,
        };

        interpreter.register_builtins(vec![
            ("putStr", Self::put_str),
            ("putStrLn", Self::put_str_ln),
            ("print", Self::print),
            ("printLn", Self::print_ln),
            ("flushStdout", Self::flush_stdout),
            ("readLine", Self::read_line),
            ("cos", Self::cos),
            ("sin", Self::sin),
            ("tan", Self::tan),
            ("acos", Self::acos),
            ("asin", Self::asin),
            ("atan", Self::atan),
            ("range", Self::range),
        ]);

        interpreter
    }

    fn register_builtins(
        &mut self,
        builtins: Vec<(
            impl ToString,
            fn(&mut Self, Vec<Box<Expr>>) -> Result<Literal, String>,
        )>,
    ) {
        for builtin in builtins {
            self.builtins.insert(builtin.0.to_string(), builtin.1);
        }
    }

    fn previous(&self) -> &Statement {
        &self.statements[self.position - 1]
    }

    fn advance(&mut self) -> Option<Statement> {
        if self.is_at_end() {
            return None;
        }

        self.position += 1;
        Some(self.previous().clone())
    }

    fn is_at_end(&self) -> bool {
        self.statements.len() > 0 && self.position == self.statements.len()
    }

    fn push(&mut self, id: String, literal: Literal) {
        let idx = self.stack.len() - 1;
        self.stack[idx].insert(id, literal);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn eval_statement(&mut self, statement: Statement) -> Result<Option<Literal>, String> {
        Ok(match statement {
            Statement::VarDef(id, expr) => self.eval_var_def(id, expr)?,
            Statement::FuncDef(id, args, statements) => self.eval_func_def(id, args, statements)?,
            Statement::Return(expr) => Some(self.get_literal(expr)?),
            Statement::While(condition, statements) => self.eval_while(condition, statements)?,
            Statement::For(id, list, statements) => self.eval_for(id, list, statements)?,
            Statement::If(condition, body, else_body) => {
                self.eval_control_flow(condition, body, else_body)?
            }
            Statement::Expr(expr) => {
                if self.context == ScopeContext::TopLevel {
                    return Err("Cannot eval expressions outside a definition.".to_string());
                }

                self.get_literal(expr)?;
                None
            }
        })
    }

    fn eval_var_def(&mut self, id: String, expr: Expr) -> Result<Option<Literal>, String> {
        if self.stack.last().unwrap().contains_key(&id) {
            return Err(format!("Variable {} already exists!", id));
        }

        let literal = self.get_literal(expr)?;
        self.push(id, literal);

        Ok(None)
    }

    fn eval_func_def(
        &mut self,
        id: String,
        args: Vec<String>,
        statements: Vec<Statement>,
    ) -> Result<Option<Literal>, String> {
        self.push(
            id.clone(),
            Literal::Function(id.clone(), args, statements.clone()),
        );

        if id.clone() == "main".to_string() {
            self.eval_call(id.clone(), vec![])?;
        }

        Ok(None)
    }

    fn eval_while(
        &mut self,
        condition: Expr,
        statements: Vec<Box<Statement>>,
    ) -> Result<Option<Literal>, String> {
        while self.eval_condition(condition.clone())? {
            for statement in statements.clone() {
                if let Statement::Return(expr) = *statement {
                    return Ok(Some(self.get_literal(expr)?));
                }

                self.eval_statement(*statement)?;
            }
        }

        Ok(None)
    }

    fn eval_for(
        &mut self,
        id: String,
        expr: Expr,
        statements: Vec<Box<Statement>>,
    ) -> Result<Option<Literal>, String> {
        match self.get_var(id.clone()) {
            Ok(_) => return Err(format!("Variable {} already exists!", id)),
            _ => (),
        }

        if let Literal::List(list) = self.get_literal(expr)? {
            for element in list {
                for statement in statements.clone() {
                    self.push(id.clone(), *element.clone());

                    self.eval_statement(*statement)?;

                    self.stack.last_mut().unwrap().remove(id.as_str());
                }
            }
        }

        Ok(None)
    }

    fn eval_control_flow(
        &mut self,
        condition: Expr,
        if_body: Vec<Box<Statement>>,
        else_body: Vec<Box<Statement>>,
    ) -> Result<Option<Literal>, String> {
        for statement in if self.eval_condition(condition)? {
            if_body
        } else {
            else_body
        } {
            if let Statement::Return(expr) = *statement {
                return Ok(Some(self.get_literal(expr)?));
            }
        }

        Ok(None)
    }

    fn eval_condition(&mut self, condition: Expr) -> Result<bool, String> {
        match self.get_literal(condition) {
            Ok(Literal::Boolean(boolean)) => Ok(boolean),
            Ok(unexpected) => Err(format!(
                "Expression must return boolean, actually return '{}'",
                unexpected
            )),
            Err(error) => Err(error),
        }
    }

    pub fn get_literal(&mut self, expr: Expr) -> Result<Literal, String> {
        Ok(match expr {
            Expr::String(string) => Literal::String(string),
            Expr::Int(int) => Literal::Int(int),
            Expr::Float(float) => Literal::Float(float),
            Expr::Boolean(boolean) => Literal::Boolean(boolean),
            Expr::Var(id) => self.get_var(id)?,
            Expr::Call(id, args) => self.eval_call(id, args)?,
            Expr::List(list) => Literal::List(self.get_literals(list)?),
            Expr::BinOp(op, left, right) => self.eval_binary_op(op, left, right)?,
        })
    }

    fn get_literals(&mut self, list: Vec<Box<Expr>>) -> Result<Vec<Box<Literal>>, String> {
        let mut literals: Vec<Box<Literal>> = vec![];

        for expr in list {
            literals.push(Box::new(self.get_literal(*expr)?));
        }

        Ok(literals)
    }

    pub fn eval_call(&mut self, id: String, call_args: Vec<Box<Expr>>) -> Result<Literal, String> {
        if self.builtins.contains_key(&id) {
            Ok(self.builtins[&id](self, call_args)?)
        } else {
            for scope in self.stack.clone().iter().rev() {
                if scope.contains_key(&id) {
                    if let Literal::Function(_, args, statements) = &scope[&id] {
                        if args.len() > call_args.len() {
                            return Err(format!("Not enought arguments for {}", id));
                        }

                        let previous_context = self.context.clone();

                        self.stack.push(HashMap::new());
                        self.context = ScopeContext::Definition;

                        for index in 0..(args.len()) {
                            let literal = self.get_literal(*(call_args[index].clone()))?;

                            self.push(args[index].clone(), literal);
                        }

                        let mut return_literal = Literal::None;

                        for statement in statements.clone() {
                            if let Some(literal) = self.eval_statement(statement)? {
                                return_literal = literal;
                                break;
                            }
                        }

                        self.context = previous_context;
                        self.pop();

                        return Ok(return_literal);
                    }
                }
            }

            Err(format!("Undefined function {}!", id))
        }
    }

    fn eval_binary_op(
        &mut self,
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<Literal, String> {
        Ok(match op {
            BinOp::Add => self.add(left, right)?,
            BinOp::Sub => self.sub(left, right)?,
            BinOp::Mul => self.mul(left, right)?,
            BinOp::Div => self.div(left, right)?,
            BinOp::Mod => self.r#mod(left, right)?,
            BinOp::Square => Literal::None, // Not implemented yet
            BinOp::Lt => self.lt(left, right)?,
            BinOp::Gt => self.gt(left, right)?,
            BinOp::Eq => self.eq(left, right)?,
            BinOp::Ne => self.ne(left, right)?,
            BinOp::Le => self.le(left, right)?,
            BinOp::Ge => self.ge(left, right)?,
        })
    }

    pub fn get_var(&self, id: String) -> Result<Literal, String> {
        for scope in self.stack.iter().rev() {
            if scope.contains_key(&id) {
                return Ok(scope[&id].clone());
            }
        }

        Err(format!("Variable {} not found!", id))
    }

    pub fn interpret(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            let statement = self.advance().unwrap();
            self.eval_statement(statement)?;
        }

        Ok(())
    }
}
