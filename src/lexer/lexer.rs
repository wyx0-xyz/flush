use super::typing::Token;

#[derive(Default)]
pub struct Lexer {
    program: String,
    tokens: Vec<Token>,
    position: usize,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        Self {
            program,
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

    fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.advance() != Some('\n') {
            self.advance();
        }
    }

    fn parse_string(&mut self) {
        let mut string = String::new();

        while !self.is_at_end() {
            match self.current() {
                Some('"') => break,
                Some(character) => string.push(character),
                None => break,
            };

            self.advance();
        }

        self.advance(); // skip "
        self.push_token(Token::String(string));
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
            Ok(int) => self.push_token(Token::Int(int)),
            Err(_) => self.push_token(Token::Float(raw_number.parse::<f32>().unwrap())),
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

        self.push_token(Token::Ident(identifier));
    }

    fn parse_token(&mut self) {
        let character = match self.advance() {
            Some(token) => token,
            None => return,
        };

        match character {
            '(' => self.push_token(Token::LParen),
            ')' => self.push_token(Token::RParen),
            '{' => self.push_token(Token::LBrace),
            '}' => self.push_token(Token::RBrace),
            '[' => self.push_token(Token::LBracket),
            ']' => self.push_token(Token::RBracket),
            ':' => self.push_token(Token::Colon),
            ';' => self.push_token(Token::Semicolon),
            ',' => self.push_token(Token::Comma),
            '+' | '-' | '*' | '/' | '%' | '^' => self.push_token(Token::Operator(character)),
            '"' => self.parse_string(),
            '#' => self.skip_comment(),
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
