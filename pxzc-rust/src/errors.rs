use std::borrow::Cow;

use crate::lexer::Token;

// Lexical Analysis Diagnostics
pub(crate) struct ScanError {
    pub line: u32,
    pub message: &'static str
}
pub(crate) type ScanResult<'a, T = Vec<Token<'a>>> = Result<T, ScanError>;

// Syntactic Analysis Diagnostics
#[derive(Debug)]
pub(crate) struct ParseError<'tokens, 'a> {
    pub token: &'tokens Token<'a>,
    pub message: &'static str
}
pub(crate) type ParseResult<'tokens, 'a, T> = Result<T, ParseError<'tokens, 'a>>;

// Runtime Diagnostics
#[derive(Debug)]
pub(crate) struct RuntimeError<'a> {
    pub token: Token<'a>,
    pub message: Cow<'a, str>
}

impl<'a> RuntimeError<'a> {
    pub(crate) fn new(token: Token<'a>, message: &'static str) -> Self {
        Self {
            token,
            message: Cow::Borrowed(message)
        }
    }

    pub(crate) fn new_owned(token: Token<'a>, message: String) -> Self {
        Self {
            token,
            message: Cow::Owned(message)
        }
    }
}

pub(crate) type RuntimeResult<'a, T> = Result<T, RuntimeError<'a>>;