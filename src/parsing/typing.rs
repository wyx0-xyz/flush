#[derive(Clone, Debug)]
pub enum Statement {
    If(Expr, Vec<Box<Statement>>, Vec<Box<Statement>>),
    VarDef(String, Expr),
    FuncDef(String, Vec<String>, Vec<Statement>),
    Return(Expr),
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
    String(String),
    Int(u32),
    Float(f32),
    Boolean(bool),
    Var(String),
    Call(String, Vec<Box<Expr>>),
    BinOp(BinOp)
}

#[derive(Clone, Debug)]
pub enum BinOp {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
}
