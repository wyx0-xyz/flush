#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Comma,          // ,
    Colon,          // :
    If,             // if
    Else,           // else
    Def,            // def
    Return,         // return
    While,          // while
    For,            // for
    In,             // in
    Break,          // break
    Load,           // load
    String(String), // "Hello, World!"
    Int(i32),       // 42
    Float(f64),     // 3.14
    Boolean(bool),  // false
    Ident(String),  // user
    Op(Op),         // + - * / % < > == /= <= >=
    Assign,         // =
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
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

#[derive(Clone, Debug)]
pub struct Token {
    pub line: usize,
    pub kind: TokenKind,
}
