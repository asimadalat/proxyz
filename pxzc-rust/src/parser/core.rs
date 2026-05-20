use std::ops::Index;
use crate::proxyz::Proxyz;
use crate::lexer::{Token, TokenType};
use crate::parser::expr::Expr;

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> ParseResult<Box<Expr<'a>>> { self.equality() }

    fn equality(&mut self) -> ParseResult<Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>> = self.comparison()?;

        while self.match_one(&[TokenType::ExclamationEqual, TokenType::EqualEqual]) {
            let operator_idx = self.previous_index();
            let right: Box<Expr<'a>> = self.comparison()?;
            let operator: &'a Token<'a> = &self.tokens[operator_idx];
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>> = self.term()?;

        while self.match_one(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr<'a>> = self.factor()?;
            let operator: &'a Token<'a> = &self.tokens[operator_idx];
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Box<Expr<'a>>> {
        if self.match_one(&[TokenType::Minus, TokenType::Exclamation]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr<'a>> = self.unary()?;
            let operator: &'a Token<'a> = &self.tokens[operator_idx];
            return Ok(Box::new(Expr::Unary {
                operator,
                operand: right
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Box<Expr<'a>>>{
        if self.match_one(&[
            TokenType::False,
            TokenType::True,
            TokenType::Null,
            TokenType::Number,
            TokenType::String
        ]) {
            let prev_idx: usize = self.previous_index();
            return Ok(Box::new(Expr::Literal { value: &self.tokens[prev_idx].literal }))
            // return Ok(Box::new(Expr::Literal { value: &self.previous().literal }))
        }

        if self.match_one(&[TokenType::LeftParen]) {
            let expression: Box<Expr> = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Box::new(Expr::Grouped { expression }))
        }

        self.error(self.peek(), "Expected expression.")?;
        Err(ParseError)
    }

    fn term(&mut self) -> ParseResult<Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>>= self.factor()?;

        while self.match_one(&[
            TokenType::Minus,
            TokenType::Plus
        ]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr<'a>> = self.factor()?;
            let operator: &'a Token<'a> = &self.tokens[operator_idx];

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>> = self.unary()?;

        while self.match_one(&[TokenType::Slash, TokenType::Star]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr> = self.unary()?;
            let operator: &Token = &self.tokens[operator_idx];

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn match_one(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check_if_type(&token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> ParseResult<&'a Token<'a>>{
        if self.check_if_type(&token_type) {
            return Ok(self.advance());
        }

        self.error(self.peek(), message)?;
        Err(ParseError)
    }

    fn check_if_type(&self, variant: &TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().variant == *variant
    }

    fn advance(&mut self) -> &'a Token<'a> {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }

    fn is_at_end(&self) -> bool { self.peek().variant == TokenType::Eof }

    fn peek(&self) -> &'a Token<'a> { self.tokens.index(self.current) }

    fn previous_index(&self) -> usize { self.current - 1 }

    fn previous(&self) -> &'a Token<'a> { self.tokens.index(self.current - 1) }

    fn error(&self, token: &Token, message: &str) -> ParseResult<()> {
        Proxyz::error_at_token(token, message);
        Err(ParseError)
    }
}

#[derive(Debug)]
struct ParseError;
type ParseResult<T> = Result<T, ParseError>;