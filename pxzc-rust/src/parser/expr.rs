use crate::lexer::Token;

pub enum Expr {
    Literal{
        token: Token
    },
    Grouped {
        expression: Box<Expr>
    },
    Unary {
        operator: Token,
        operand: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    }
}