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
}
