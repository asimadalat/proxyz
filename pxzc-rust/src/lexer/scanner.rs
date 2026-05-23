use phf::phf_map;

use crate::errors::{ScanError, ScanResult};
use crate::lexer::{TokenKind, Literal, Token};

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "null" => TokenKind::Null,
    "val" => TokenKind::Val,
    "var" => TokenKind::Var,
    "struct" => TokenKind::Struct,
    "fn" => TokenKind::Fn,
    "as" => TokenKind::As,
    "is" => TokenKind::Is,
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "for" => TokenKind::For,
    "while" => TokenKind::While,
    "in" => TokenKind::In,
    "return" => TokenKind::Return,
    "and" => TokenKind::And,
    "or" => TokenKind::Or,
    "unsafe" => TokenKind::Unsafe,
    "module" => TokenKind::Module,
    "import" => TokenKind::Import,
    "export" => TokenKind::Export
};

pub fn parse_keyword(keyword: &str) -> Option<TokenKind> {
    KEYWORDS.get(keyword).cloned()
}

pub struct Scanner<'a> {
    tokens: Vec<Token<'a>>,
    start: usize,
    position: usize,
    line: u32,
    prev_token_type: Option<TokenKind>,
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'_ str) -> Scanner<'_> {
        Scanner {
            tokens: Vec::new(),
            start: 0,
            position: 0,
            line: 1,
            prev_token_type: None,
            source,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> ScanResult<'a> {
        while !self.is_at_end() {
            self.start = self.position;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenKind::Eof,
            "",
            self.line,
            Literal::None,
        ));

        let scanned_tokens = std::mem::take(&mut self.tokens);

        Ok(scanned_tokens)
    }

    fn scan_token(&mut self) -> ScanResult<'a, ()> {
        let c: char = self.proceed();
        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            ':' => self.add_token(TokenKind::Colon),
            '?' => {
                let token_type = if (&mut *self).then('?') {
                    TokenKind::QuestionQuestion
                } else if (&mut *self).then('=') {
                    TokenKind::QuestionEqual
                } else { TokenKind::Question };
                (&mut *self).add_token(token_type);
            }
            '.' => {
                let token_type = if self.then('.') {
                    TokenKind::DotDot
                } else { TokenKind::Dot };
                self.add_token(token_type);
            }
            '-' => {
                let token_type = if self.then('=') {
                    TokenKind::MinusEqual
                } else { TokenKind::Minus };
                self.add_token(token_type);
            }
            '+' => {
                let token_type = if self.then('=') {
                    TokenKind::PlusEqual
                } else { TokenKind::Plus };
                self.add_token(token_type);
            }
            '*' => {
                let token_type = if self.then('=') {
                    TokenKind::StarEqual
                } else { TokenKind::Star };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if (&mut *self).then('=') {
                    TokenKind::EqualEqual
                } else if (&mut *self).then('>') {
                    TokenKind::FatArrow
                } else { TokenKind::Equal };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.then('=') {
                    TokenKind::GreaterEqual
                } else { TokenKind::Greater };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.then('=') {
                    TokenKind::LessEqual
                } else { TokenKind::Less };
                self.add_token(token_type);
            }
            '!' => {
                let token_type = if (&mut *self).then('=') {
                    TokenKind::ExclamationEqual
                } else { TokenKind::Exclamation };
                (&mut *self).add_token(token_type);
            }
            '/' => {
                if self.then('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.proceed();
                    }
                } else if self.then('=') {
                    self.add_token(TokenKind::SlashEqual)
                } else {
                    self.add_token(TokenKind::Slash)
                }
            }
            '"' => self.string()?,
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\r' | '\t' => { /* Ignore whitespace characters */ }
            '\n' => {
                self.line += 1;

                match self.prev_token_type {
                    Some(TokenKind::Identifier) |
                    Some(TokenKind::Number) |
                    Some(TokenKind::String) |
                    Some(TokenKind::RightParen) |
                    Some(TokenKind::RightBrace) |
                    Some(TokenKind::RightBracket) => {
                        self.add_token(TokenKind::NewLine)
                    },
                    _ => { }
                }
            },
            _ => return Err(ScanError {
                line: self.line,
                message: "Unexpected character.",
            })
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.proceed();
        }

        let text = &self.source[self.start..self.position];
        let token_type = parse_keyword(text).unwrap_or(TokenKind::Identifier);

        self.add_token(token_type);
    }

    fn string(&mut self) -> ScanResult<'a, ()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.proceed();
        }

        if self.is_at_end() {
            return Err(ScanError {
                line: self.line,
                message: "Unterminated string literal.",
            });
        }

        self.proceed();
        let raw_string = &self.source[self.start + 1..self.position];

        self.add_token_with_literal(TokenKind::String, Literal::String(raw_string));

        Ok(())
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
            TokenKind::Number,
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

    fn add_token(&mut self, kind: TokenKind) {
        self.prev_token_type = Some(kind);
        self.add_token_with_literal(kind, Literal::None);
    }

    fn add_token_with_literal(&mut self, kind: TokenKind, literal: Literal<'a>) {
        let text: &'a str = &self.source[self.start..self.position];
        self.tokens
            .push(Token::new(kind, text, self.line, literal));
    }
}
