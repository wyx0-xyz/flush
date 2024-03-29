use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    If(Expr, Vec<Box<Statement>>, Vec<Box<Statement>>), // if (...) { ... }
    VarDef(String, Expr),                               // def user_id = ...
    FuncDef(String, Vec<String>, Vec<Statement>),       // def factorial(...) { ... }
    VarSet(String, Expr),                               // user_id = ...
    IndexSet(Expr, Expr, Expr),                         // {}["name"] = marcello
    Return(Expr),                                       // return ...
    While(Expr, Vec<Box<Statement>>),                   // while (true) { ... }
    For(String, Expr, Vec<Box<Statement>>),             // for (i in ...) { ... }
    Break,                                              // break
    Load(String),                                       // load "..."
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    String(String),                          // "Hello, flush!"
    Int(i32),                                // -32
    Float(f64),                              // 18.25
    Boolean(bool),                           // false
    Var(String),                             // userId
    Call(String, Vec<Box<Expr>>),            // printLn(...)
    List(Vec<Box<Expr>>),                    // [1, 2, 3]
    Dictionnary(HashMap<String, Box<Expr>>), // { "key": "value" }
    Index(Box<Expr>, Box<Expr>),             // [1, 2, 3, 4][0] or {"name": "flush"}["name"]
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    FloorDiv, // //
    Mod,      // %
    Pow,      // ^
    Lt,       // <
    Gt,       // >
    Eq,       // ==
    Ne,       // /=
    Le,       // <=
    Ge,       // >=
}
