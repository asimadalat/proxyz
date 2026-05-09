use std::io::Error;
use crate::expression::binary::Binary;
use crate::expression::grouped::Grouped;
use crate::expression::literal::Literal;
use crate::expression::unary::Unary;

pub trait Visitor<T> {
    fn visit_literal(&mut self, literal: Literal) -> Result<T, Error>;
    fn visit_grouped(&mut self, grouped: Grouped) -> Result<T, Error>;
    fn visit_unary(&mut self, unary: Unary) -> Result<T, Error>;
    fn visit_binary(&mut self, binary: Binary) -> Result<T, Error>;
}