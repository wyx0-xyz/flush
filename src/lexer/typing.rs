#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Colon,          // :
    Semicolon,      // ;
    Comma,          // ,
    Def,            // def
    String(String), // "Hello, World!"
    Int(u32),       // 42
    Float(f32),     // 3.14
    Ident(String),  // user
    Operator(char), // +
}

#[derive(Clone, Debug)]
pub struct Token {
    pub line: usize,
    pub kind: TokenKind
}
