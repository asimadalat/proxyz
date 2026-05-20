use std::fmt;
use std::fmt::Formatter;
use crate::lexer::{TokenType, Literal};

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub(crate) variant: TokenType,
    pub(crate) lexeme: &'a str,
    pub(crate) literal: Literal<'a>,
    pub(crate) line: u32,
}

impl<'a> Token<'a> {
    pub fn new(variant: TokenType, lexeme: &'a str, line: u32, literal: Literal<'a>) -> Self {
        Token {
            variant,
            lexeme,
            line,
            literal,
        }
    }

    pub fn lexeme(&self) -> &'a str { self.lexeme }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}