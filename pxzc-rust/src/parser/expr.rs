use crate::lexer::{Literal, Token};

pub enum Expr<'a> {
    Literal{
        value: &'a Literal<'a>
    },
    Grouped {
        expression: Box<Expr<'a>>
    },
    Unary {
        operator: &'a Token<'a>,
        operand: Box<Expr<'a>>
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>
    }
}