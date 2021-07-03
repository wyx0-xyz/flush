#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Colon,          // :
    Comma,          // ,
    Def,            // def
    Return,         // return
    String(String), // "Hello, World!"
    Int(u32),       // 42
    Float(f32),     // 3.14
    Boolean(bool),  // false
    Ident(String),  // user
    OpAdd,          // +
    OpSub,          // -
    OpMul,          // *
    OpDiv,          // /
    OpMod,          // %
    Equal,          // =
}

#[derive(Clone, Debug)]
pub struct Token {
    pub line: usize,
    pub kind: TokenKind
}
