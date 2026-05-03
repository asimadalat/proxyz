#[derive(Debug, Clone)]
pub enum TokenType {
    // Literals
    Number,
    String,
    Identifier,

    // Indentation
    Indent,
    Dedent,

    // Single-character generics
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Dot,
    Comma,
    Colon,
    Question,
    Plus,
    Minus,
    Star,
    Slash,

    // Assignment operators
    Equal,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,

    // Range & Comparison Operators
    DotDot,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    FatArrow,

    //  Reserved Keywords
    Let,
    Struct,
    Extend,
    Unsafe,
    Module,
    Import,
    Export,
    Fn,
    As,
    If,
    Elif,
    Else,
    For,
    While,
    In,
    Return,
    And,
    Or,
    Not,
    Is,
    Take,
    Lent,
    Consume,
    True,
    False,

    Eof,
}
