use phf::phf_map;
use std::iter::Iterator;

use crate::driver::Proxyz;
use crate::lexer::{TokenType, Literal, Token};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "let" => TokenType::Let,
    "struct" => TokenType::Struct,
    "extend" => TokenType::Extend,
    "unsafe" => TokenType::Unsafe,
    "module" => TokenType::Module,
    "import" => TokenType::Import,
    "export" => TokenType::Export,
    "fn" => TokenType::Fn,
    "as" => TokenType::As,
    "if" => TokenType::If,
    "elif" => TokenType::Elif,
    "else" => TokenType::Else,
    "for" => TokenType::For,
    "while" => TokenType::While,
    "in" => TokenType::In,
    "return" => TokenType::Return,
    "and" => TokenType::And,
    "or" => TokenType::Or,
    "not" => TokenType::Not,
    "is" => TokenType::Is,
    "take" => TokenType::Take,
    "lent" => TokenType::Lent,
    "consume" => TokenType::Consume,
    "true" => TokenType::True,
    "false" => TokenType::False
};

pub fn parse_keyword(keyword: &str) -> Option<TokenType> {
    KEYWORDS.get(keyword).cloned()
}

pub struct Scanner {
    tokens: Vec<Token>,
    start: usize,
    position: usize,
    line: usize,
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            tokens: Vec::new(),
            start: 0,
            position: 0,
            line: 1,
            source,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.position;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            self.line,
            Literal::None,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.proceed();
        match c {
            '(' => self.add_token(TokenType::OpenBracket),
            ')' => self.add_token(TokenType::CloseBracket),
            '{' => self.add_token(TokenType::OpenBrace),
            '}' => self.add_token(TokenType::CloseBrace),
            ',' => self.add_token(TokenType::Comma),
            '?' => self.add_token(TokenType::Question),
            ':' => self.add_token(TokenType::Colon),
            '.' => {
                let token_type = if self.then('.') {
                    TokenType::DotDot
                } else {
                    TokenType::Dot
                };
                self.add_token(token_type);
            }
            '-' => {
                let token_type = if self.then('=') {
                    TokenType::MinusEqual
                } else {
                    TokenType::Minus
                };
                self.add_token(token_type);
            }
            '+' => {
                let token_type = if self.then('=') {
                    TokenType::PlusEqual
                } else {
                    TokenType::Plus
                };
                self.add_token(token_type);
            }
            '*' => {
                let token_type = if self.then('=') {
                    TokenType::StarEqual
                } else {
                    TokenType::Star
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.then('>') {
                    TokenType::FatArrow
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.then('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.then('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.then('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.proceed();
                    }
                } else if self.then('=') {
                    self.add_token(TokenType::SlashEqual)
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\r' | '\t' => { /* Ignore whitespace characters */ }
            '\n' => self.line += 1,
            _ => Proxyz::exception(self.line, "Unexpected character."),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.proceed();
        }

        let text = &self.source[self.start..self.position];
        let token_type = parse_keyword(text).unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.proceed();
        }

        if self.is_at_end() {
            Proxyz::exception(self.line, "Unterminated string literal.");
            return;
        }

        self.proceed();
        let raw_string = self.source[self.start + 1..self.position].to_string();

        self.add_token_with_literal(TokenType::String, Literal::String(raw_string));
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.proceed();
        }

        // Check for decimal
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            // Consume the '.'
            self.proceed();

            while Scanner::is_digit(self.peek()) {
                self.proceed();
            }
        }

        self.add_token_with_literal(
            TokenType::Number,
            Literal::Number(self.source[self.start..self.position].parse().unwrap()),
        );
    }

    fn then(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.position) == Some(expected) {
            self.position += 1;
            return true;
        }
        false
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.position).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.position + 1).unwrap()
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn proceed(&mut self) -> char {
        self.position += 1;
        self.source.chars().nth(self.position - 1).unwrap()
    }

    fn add_token(&mut self, variant: TokenType) {
        self.add_token_with_literal(variant, Literal::None);
    }

    fn add_token_with_literal(&mut self, variant: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.position];
        self.tokens
            .push(Token::new(variant, text.to_string(), self.line, literal));
    }
}
