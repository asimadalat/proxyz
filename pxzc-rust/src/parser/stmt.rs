use crate::lexer::Token;
use crate::parser::expr::Expr;

pub enum Stmt<'a> {
    Expression(Box<Expr<'a>>),
    Log(Box<Expr<'a>>),
    Var(Token<'a>, Option<Box<Expr<'a>>>)
}