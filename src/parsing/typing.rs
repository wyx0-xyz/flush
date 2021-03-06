#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    If(Expr, Vec<Box<Statement>>, Vec<Box<Statement>>), // if (...) { ... }
    VarDef(String, Expr),                               // def user_id = ...
    FuncDef(String, Vec<String>, Vec<Statement>),       // def factorial(...) { ... }
    VarSet(String, Expr),                               // set user_id = ...
    Return(Expr),                                       // return ...
    While(Expr, Vec<Box<Statement>>),                   // while (true) { ... }
    For(String, Expr, Vec<Box<Statement>>),             // for (i in ...) { ... }
    Break,                                              // break
    Load(String),                                       // load "..."
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    String(String),               // "Hello, flush!"
    Int(i32),                     // -32
    Float(f64),                   // 18.25
    Boolean(bool),                // false
    Var(String),                  // userId
    Call(String, Vec<Box<Expr>>), // printLn(...)
    List(Vec<Box<Expr>>),         // [1, 2, 3]
    ListAt(Box<Expr>, Box<Expr>), // [1, 2, 3]@3
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
