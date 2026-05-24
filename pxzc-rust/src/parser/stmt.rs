use crate::parser::expr::Expr;

pub enum Stmt<'a> {
    Expression(Box<Expr<'a>>),
    Log(Box<Expr<'a>>)
}