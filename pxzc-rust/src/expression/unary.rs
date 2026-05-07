use crate::lexer::Token;
use crate::expression::expr::Expr;

pub struct Unary {
    pub operator: Token,
    pub operand: Box<Expr>
}