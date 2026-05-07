use crate::lexer::Token;
use crate::expression::expr::Expr;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}