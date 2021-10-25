use crate::flush::run;
use crate::interpreting::typing::*;
use crate::parsing::typing::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Interpreter<'a> {
    statements: Vec<Statement>,
    file_path: PathBuf,
    cache: &'a mut Vec<PathBuf>,
    eval_main: bool,
    stack: Vec<HashMap<String, Literal>>,
    builtins: HashMap<String, fn(&mut Self, Vec<Box<Expr>>) -> Result<Literal, String>>,
    contexts: Vec<Context>,
    loops_conditions: Vec<Expr>,
    position: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(
        statements: Vec<Statement>,
        file_path: PathBuf,
        cache: &'a mut Vec<PathBuf>,
        eval_main: bool,
    ) -> Self {
        let mut interpreter = Self {
            statements,
            file_path,
            cache,
            eval_main,
            stack: vec![HashMap::new()],
            builtins: HashMap::new(),
            contexts: vec![Context::TopLevel],
            loops_conditions: vec![],
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
            ("random", Self::random),
            ("parseInt", Self::parse_int),
            ("parseFloat", Self::parse_float),
            ("toString", Self::to_string),
        ]);

        interpreter
    }

    pub fn get_stack(&self) -> HashMap<String, Literal> {
        self.stack[0].clone()
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

    pub fn eval_statement(&mut self, statement: Statement) -> Result<Option<Literal>, String> {
        Ok(match statement {
            Statement::VarDef(id, expr) => self.eval_var_def(id, expr)?,
            Statement::FuncDef(id, args, statements) => self.eval_func_def(id, args, statements)?,
            Statement::VarSet(id, expr) => self.eval_var_set(id, expr)?,
            Statement::Return(expr) => Some(self.get_literal(expr)?),
            Statement::While(condition, statements) => self.eval_while(condition, statements)?,
            Statement::For(id, list, statements) => self.eval_for(id, list, statements)?,
            Statement::Break => self.eval_break()?,
            Statement::Load(file_path) => self.eval_load(file_path)?,
            Statement::If(condition, body, else_body) => {
                self.eval_control_flow(condition, body, else_body)?
            }
            Statement::Expr(expr) => {
                if !self.contexts.contains(&Context::Function) {
                    return Err("Cannot eval expressions outside a function.".to_string());
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

        if self.eval_main && id.clone() == "main".to_string() {
            self.eval_call(id.clone(), vec![])?;
        }

        Ok(None)
    }

    fn eval_var_set(&mut self, id: String, expr: Expr) -> Result<Option<Literal>, String> {
        let literal = self.get_literal(expr)?;

        for stack in self.stack.iter_mut().rev() {
            if stack.clone().contains_key(&*id) {
                stack.insert(id, literal);
                return Ok(None);
            }
        }

        Err(format!("Variable {} not found!", id))
    }

    fn eval_while(
        &mut self,
        condition: Expr,
        statements: Vec<Box<Statement>>,
    ) -> Result<Option<Literal>, String> {
        if !self.eval_condition(condition.clone())? {
            return Ok(None);
        }

        let mut result: Option<Literal> = None;

        self.stack.push(HashMap::new());
        self.contexts.push(Context::Loop);
        self.loops_conditions.push(condition.clone());

        'main_while: loop {
            for statement in statements.clone() {
                if !self.eval_condition(self.loops_conditions.last().unwrap().clone())? {
                    break 'main_while;
                }

                if let Statement::Return(expr) = *statement {
                    result = Some(self.get_literal(expr)?);
                    break 'main_while;
                }

                self.eval_statement(*statement)?;
            }
        }

        self.stack.pop();
        self.contexts.pop();
        self.loops_conditions.pop();

        Ok(result)
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
            self.stack.push(HashMap::new());
            self.contexts.push(Context::Loop);
            self.loops_conditions.push(Expr::Boolean(true));

            for element in list {
                if !self.eval_condition(self.loops_conditions.last().unwrap().clone())? {
                    break;
                }

                for statement in statements.clone() {
                    if Statement::Break == *statement {
                        return Ok(None);
                    }

                    self.push(id.clone(), *element.clone());

                    match self.eval_statement(*statement) {
                        Ok(_) => {}
                        Err(e) => {
                            if e != "Can only use break in loops!".to_string() {
                                return Err(e);
                            }
                        }
                    }

                    self.stack.last_mut().unwrap().remove(id.as_str());
                }
            }

            self.stack.pop();
            self.contexts.pop();
            self.loops_conditions.pop();
        }

        Ok(None)
    }

    fn eval_break(&mut self) -> Result<Option<Literal>, String> {
        if self.contexts.contains(&Context::Loop) {
            let last_index = self.loops_conditions.len() - 1;
            self.loops_conditions[last_index] = Expr::Boolean(false);

            return Ok(None);
        }

        Err("Can only use break in loops!".to_string())
    }

    fn eval_load(&mut self, raw_file_path: String) -> Result<Option<Literal>, String> {
        if raw_file_path == self.file_path.to_string_lossy().to_string() {
            return Ok(None);
        }

        match run(&raw_file_path, self.cache) {
            Ok(Some(stack)) => {
                for (id, value) in stack {
                    self.push(id, value);
                }
            }
            Ok(None) => {}
            Err(e) => return Err(e),
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
            } else {
                self.eval_statement(*statement)?;
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
            Expr::ListAt(list, index) => self.eval_list_at(list, index)?,
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

                        self.stack.push(HashMap::new());
                        self.contexts.push(Context::Function);

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

                        self.stack.pop();
                        self.contexts.pop();

                        return Ok(return_literal);
                    }
                }
            }

            Err(format!("Undefined function {}!", id))
        }
    }

    fn eval_list_at(&mut self, list: Box<Expr>, index: Box<Expr>) -> Result<Literal, String> {
        let list = match self.get_literal(*list)? {
            Literal::List(list) => list,
            unexpected => return Err(format!("Couldn't index '{}'", unexpected)),
        };
        let index = match self.get_literal(*index)? {
            Literal::Int(int) => int,
            unexpected => return Err(format!("Expected Integer found '{:?}'", unexpected)),
        };

        if index < 0 {
            return Err("Couldn't index list with negatives integers".to_string());
        }

        let index = index as usize;

        if index >= list.len() {
            return Err("List index out of range".to_string());
        }

        Ok(*list[index].clone())
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
            BinOp::Div => self.div(left, right, false)?,
            BinOp::FloorDiv => self.div(left, right, true)?,
            BinOp::Mod => self.r#mod(left, right)?,
            BinOp::Pow => self.pow(left, right)?,
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
