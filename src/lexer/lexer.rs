use super::typing::{Token, TokenKind};

#[derive(Default)]
pub struct Lexer {
    program: String,
    tokens: Vec<Token>,
    position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        Self {
            program,
            line: 1,
            ..Default::default()
        }
    }

    fn previous(&self) -> Option<char> {
        self.program.chars().nth(self.position - 1)
    }

    fn current(&self) -> Option<char> {
        self.program.chars().nth(self.position)
    }

    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.program.len()
    }

    fn push_token(&mut self, token: TokenKind) {
        self.tokens.push(Token {
            line: self.line,
            kind: token,
        });
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.advance() != Some('\n') {
            self.advance();
        }

        self.line += 1;
    }

    fn parse_string(&mut self) {
        let mut string = String::new();

        while !self.is_at_end() && self.current() != Some('"') {
            match self.current() {
                Some(character) => string.push(character),
                None => break,
            };

            self.advance();
        }

        if self.current() != Some('"') {
            panic!("Unclosed string found!")
        }

        self.advance(); // skip "
        self.push_token(TokenKind::String(string));
    }

    fn parse_number(&mut self) {
        let mut raw_number = String::from(self.previous().unwrap());

        while !self.is_at_end() {
            let current = match self.current() {
                Some(character) => character,
                None => break,
            };

            if current == '.' || current.is_ascii_digit() {
                raw_number.push(current);
                self.advance();
            } else {
                break;
            }
        }

        match raw_number.parse::<u32>() {
            Ok(int) => self.push_token(TokenKind::Int(int)),
            Err(_) => self.push_token(TokenKind::Float(raw_number.parse::<f32>().unwrap())),
        }
    }

    fn parse_identifier(&mut self) {
        let mut identifier = String::from(self.previous().unwrap());

        while !self.is_at_end() {
            let current = match self.current() {
                Some(character) => character,
                None => break,
            };

            if current.is_ascii_alphanumeric() {
                identifier.push(current);
                self.advance();
            } else {
                break;
            }
        }

        let token = match identifier.as_str() {
            "def" => TokenKind::Def,
            ident => TokenKind::Ident(ident.to_string()),
        };

        self.push_token(token);
    }

    fn parse_token(&mut self) {
        let character = match self.advance() {
            Some(token) => token,
            None => return,
        };

        match character {
            '(' => self.push_token(TokenKind::LParen),
            ')' => self.push_token(TokenKind::RParen),
            '{' => self.push_token(TokenKind::LBrace),
            '}' => self.push_token(TokenKind::RBrace),
            '[' => self.push_token(TokenKind::LBracket),
            ']' => self.push_token(TokenKind::RBracket),
            ':' => self.push_token(TokenKind::Colon),
            ';' => self.push_token(TokenKind::Semicolon),
            ',' => self.push_token(TokenKind::Comma),
            '+' | '-' | '*' | '/' | '%' | '^' | '=' => {
                self.push_token(TokenKind::Operator(character))
            }
            '"' => self.parse_string(),
            '#' => self.skip_comment(),
            '\n' => self.line += 1,
            _ if character.is_ascii_digit() => self.parse_number(),
            _ if character.is_ascii_alphanumeric() => self.parse_identifier(),
            _ => (),
        };
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.parse_token();
        }

        &self.tokens
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::lexer::typing::*;

    fn get_types(tokens: &Vec<Token>) -> Vec<TokenKind> {
        tokens.into_iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn single_line_comment() {
        let mut lexer = Lexer::new("# hello, world\n#lorem".to_string());
        assert_eq!(get_types(lexer.tokenize()), vec![])
    }

    #[test]
    fn parentheses_braces_brackets() {
        let mut lexer = Lexer::new("( } [ ) { ]".to_string());
        assert_eq!(
            get_types(lexer.tokenize()),
            vec![
                TokenKind::LParen,
                TokenKind::RBrace,
                TokenKind::LBracket,
                TokenKind::RParen,
                TokenKind::LBrace,
                TokenKind::RBracket
            ]
        )
    }

    #[test]
    fn punctuation() {
        let mut lexer = Lexer::new("; , :".to_string());
        assert_eq!(
            get_types(lexer.tokenize()),
            vec![TokenKind::Semicolon, TokenKind::Comma, TokenKind::Colon]
        )
    }

    #[test]
    fn string() {
        let mut lexer = Lexer::new(r#""Hello, World!""#.to_string());
        assert_eq!(
            get_types(lexer.tokenize()),
            vec![TokenKind::String("Hello, World!".to_string())]
        );
    }

    #[test]
    #[should_panic(expected = "Unclosed string found!")]
    fn unclosed_string() {
        let mut lexer = Lexer::new(r#""Hello flush"#.to_string());
        lexer.tokenize();
    }

    #[test]
    fn numbers() {
        let mut lexer = Lexer::new("32 18.25".to_string());
        assert_eq!(
            get_types(lexer.tokenize()),
            vec![TokenKind::Int(32), TokenKind::Float(18.25)]
        )
    }

    #[test]
    fn keywords() {
        let mut lexer = Lexer::new("def user".to_string());
        assert_eq!(
            get_types(lexer.tokenize()),
            vec![TokenKind::Def, TokenKind::Ident("user".to_string())]
        )
    }

    #[test]
    fn operators() {
        let mut lexer = Lexer::new("+/*-=%".to_string());
        assert_eq!(
            get_types(lexer.tokenize()),
            vec![
                TokenKind::Operator('+'),
                TokenKind::Operator('/'),
                TokenKind::Operator('*'),
                TokenKind::Operator('-'),
                TokenKind::Operator('='),
                TokenKind::Operator('%'),
            ]
        )
    }
}
