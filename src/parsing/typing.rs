#[derive(Clone, Debug)]
pub enum Statement {
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
    Call(String, Vec<Expr>),
}
