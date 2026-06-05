use crate::lexer::{Literal, Token};

pub enum Expr<'a> {
    Literal{
        value: Literal<'a>
    },
    Grouped {
        expression: Box<Expr<'a>>
    },
    Unary {
        operator: Token<'a>,
        operand: Box<Expr<'a>>
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>
    },
    Variable {
        name: Token<'a>
    },
    Assign {
        name: Token<'a>,
        value: Box<Expr<'a>>
    }
}