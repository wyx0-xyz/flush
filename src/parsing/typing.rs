#[derive(Clone, Debug)]
pub enum Statement {
    If(Expr, Vec<Box<Statement>>, Vec<Box<Statement>>), // if (...) { ... }
    VarDef(String, Expr),                               // def user_id = ...
    FuncDef(String, Vec<String>, Vec<Statement>),       // def factorial(...) { ... }
    Return(Expr),                                       // return ...
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
    String(String),               // "Hello, flush!"
    Int(u32),                     // 32
    Float(f32),                   // 18.25
    Boolean(bool),                // false
    Var(String),                  // userId
    Call(String, Vec<Box<Expr>>), // printLn(...)
    BinOp(BinOp),
}

#[derive(Clone, Debug)]
pub enum BinOp {
    Add(Box<Expr>, Box<Expr>), // +
    Sub(Box<Expr>, Box<Expr>), // -
    Mul(Box<Expr>, Box<Expr>), // *
    Div(Box<Expr>, Box<Expr>), // /
    Mod(Box<Expr>, Box<Expr>), // %
    Lt(Box<Expr>, Box<Expr>),  // <
    Gt(Box<Expr>, Box<Expr>),  // >
    Eq(Box<Expr>, Box<Expr>),  // ==
    Le(Box<Expr>, Box<Expr>),  // <=
    Ge(Box<Expr>, Box<Expr>),  // >=
}
