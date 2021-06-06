#[derive(Debug)]
pub enum Token {
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
    Ident(String),  // def
    Operator(char), // +
}
