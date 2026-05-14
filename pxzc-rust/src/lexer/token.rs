use std::fmt;
use std::fmt::Formatter;
use crate::lexer::{TokenType, Literal};

#[derive(Debug)]
pub struct Token {
    variant: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub(crate) fn new(variant: TokenType, lexeme: String, line: usize, literal: Literal) -> Self {
        Token {
            variant,
            lexeme,
            line,
            literal,
        }
    }

    pub fn lexeme(&self) -> &str { &self.lexeme }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}