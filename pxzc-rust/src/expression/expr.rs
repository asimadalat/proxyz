use crate::expression::binary::Binary;
use crate::expression::grouped::Grouped;
use crate::expression::literal::Literal;
use crate::expression::unary::Unary;

pub enum Expr {
    Literal(Literal),
    Grouped(Grouped),
    Unary(Unary),
    Binary(Binary)
}