use crate::lexer::Token;

// Lexical Analysis Diagnostics
pub struct ScanError {
    pub line: u32,
    pub message: &'static str
}

pub type ScanResult<'a, T = Vec<Token<'a>>> = Result<T, ScanError>;

// Syntactic Analysis Diagnostics
#[derive(Debug)]
pub(crate) struct ParseError<'a> {
    pub token: &'a Token<'a>,
    pub message: &'static str
}
pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;
