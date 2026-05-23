#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Literals
    Number,
    String,
    Null,
    True,
    False,
    Identifier,

    // Structural Symbols
    LeftParen,          // (
    RightParen,         // )
    LeftBracket,        // [
    RightBracket,       // [
    LeftBrace,          // {
    RightBrace,         // }
    Dot,
    Comma,
    Colon,
    Question,

    // Mathematical Operators
    Plus,
    Minus,
    Star,
    Slash,

    // Assignment Operators
    Equal,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
    QuestionEqual,

    // Range & Comparison Operators
    DotDot,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    ExclamationEqual,
    Exclamation,
    QuestionQuestion,

    FatArrow, // =>

    //  Reserved Keywords
    Val,
    Var,
    Struct,
    Fn,
    As,
    Is,

    If,
    Else,
    For,
    While,
    In,
    Return,

    And,
    Or,

    Unsafe,
    Module,
    Import,
    Export,

    NewLine,
    Eof,
}
