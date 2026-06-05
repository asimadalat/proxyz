use std::ops::Index;

use crate::errors::{ParseError, ParseResult};
use crate::lexer::{Token, TokenKind};
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;

pub struct Parser<'tokens, 'a> {
    tokens: &'tokens [Token<'a>],
    current: usize
}

impl<'tokens, 'a> Parser<'tokens, 'a> {
    pub fn new(tokens: &'tokens [Token<'a>]) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> ParseResult<'tokens, 'a, Vec<Stmt<'a>>> {
        let mut statements: Vec<Stmt<'a>> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> ParseResult<'tokens, 'a, Stmt<'a>> {
        match self.match_one(&[TokenKind::Var]) {
            true => self.var_declaration()
                .map_err(|e| {
                    self.synchronise();
                    e
                }),
            false => self.statement()
                .map_err(|e| {
                    self.synchronise();
                    e
                })
        }
    }

    fn expression(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> { self.assignment() }

    fn statement(&mut self) -> ParseResult<'tokens, 'a, Stmt<'a>> {
        if self.match_one(&[TokenKind::Log]) {
            return Ok(self.log_statement()?);
        }

        Ok(self.expression_statement()?)
    }

    fn log_statement(&mut self) -> ParseResult<'tokens, 'a, Stmt<'a>> {
        let value: Box<Expr> = self.expression()?;
        self.consume_terminator(
            "Expected newline after value."
        )?;

        Ok(Stmt::Log(value))
    }


    fn expression_statement(&mut self) -> ParseResult<'tokens, 'a, Stmt<'a>> {
        let expr: Box<Expr> = self.expression()?;
        self.consume_terminator(
            "Expected newline after expression."
        )?;

        Ok(Stmt::Expression(expr))
    }

    fn var_declaration(&mut self) -> ParseResult<'tokens, 'a, Stmt<'a>> {
        let name_ref = self.consume(
            TokenKind::Identifier,
            "Expected variable name."
        )?;

        let name: Token<'a> = *name_ref;

        let mut initialiser = None;
        if self.match_one(&[TokenKind::Equal]) {
            initialiser = Some(self.expression()?);
        }

        self.consume_terminator(
            "Expected newline after variable declaration."
        )?;

        Ok(Stmt::Var(name, initialiser))
    }
    
    fn assignment(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> {
        let expression = self.equality()?;

        if self.match_one(&[TokenKind::Equal]) {
            let equals_idx = self.previous_index();
            let value = self.assignment()?;

            if let Expr::Variable { name } = *expression {
                return Ok(Box::new(Expr::Assign { name, value }));
            }

            return Err(Self::error(
                &self.tokens[equals_idx],
                "Invalid assignment target.",
            ));
        }
        
        Ok(expression)
    }

    fn equality(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>> = self.comparison()?;

        while self.match_one(&[TokenKind::ExclamationEqual, TokenKind::EqualEqual]) {
            let operator_idx = self.previous_index();
            let right: Box<Expr<'a>> = self.comparison()?;
            let operator: Token<'a> = self.tokens[operator_idx];
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>> = self.term()?;

        while self.match_one(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual
        ]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr<'a>> = self.factor()?;
            let operator: Token<'a> = self.tokens[operator_idx];
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> {
        if self.match_one(&[TokenKind::Minus, TokenKind::Exclamation]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr<'a>> = self.unary()?;
            let operator: Token<'a> = self.tokens[operator_idx];
            return Ok(Box::new(Expr::Unary {
                operator,
                operand: right
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>>{
        if self.match_one(&[
            TokenKind::False,
            TokenKind::True,
            TokenKind::Null,
            TokenKind::Number,
            TokenKind::String
        ]) {
            let prev_token_idx = self.previous_index();
            return Ok(Box::new(Expr::Literal {
                value: self.tokens[prev_token_idx].literal
            }))
        }

        if self.match_one(&[TokenKind::LeftParen]) {
            let expression: Box<Expr> = self.expression()?;
            self.consume(TokenKind::RightParen, "Expected ')' after expression.")?;
            return Ok(Box::new(Expr::Grouped { expression }))
        }

        if self.match_one(&[TokenKind::Identifier]) {
            let prev_token_idx = self.previous_index();
            return Ok(Box::new(Expr::Variable {
                name: *self.tokens.index(prev_token_idx)
            }));
        }

        Err(Self::error(
            self.peek(),
            "Expected expression."
        ))
    }

    fn term(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>>= self.factor()?;

        while self.match_one(&[
            TokenKind::Minus,
            TokenKind::Plus
        ]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr<'a>> = self.factor()?;
            let operator: Token<'a> = self.tokens[operator_idx];

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<'tokens, 'a, Box<Expr<'a>>> {
        let mut expr: Box<Expr<'a>> = self.unary()?;

        while self.match_one(&[TokenKind::Slash, TokenKind::Star]) {
            let operator_idx: usize = self.previous_index();
            let right: Box<Expr> = self.unary()?;
            let operator: Token = self.tokens[operator_idx];

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn match_one(&mut self, types: &[TokenKind]) -> bool {
        for token_type in types {
            if self.check_if_type(&token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(
        &mut self,
        token_type: TokenKind,
        message: &'static str
    ) -> ParseResult<'tokens, 'a, &'tokens Token<'a>>
    {
        if self.check_if_type(&token_type) {
            return Ok(self.advance());
        }

        Err(Self::error(self.peek(), message))
    }

    fn consume_terminator(
        &mut self,
        error_message: &'static str
    ) -> ParseResult<'tokens, 'a, ()> {
        if self.is_at_end() {
            if self.check_if_type(&TokenKind::Eof) {
                self.advance();
            }
            return Ok(());
        }

        if self.check_if_type(&TokenKind::NewLine) {
            self.advance();
            return Ok(());
        }

        Err(Self::error(self.peek(), error_message))
    }

    fn check_if_type(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() { return false; }
        self.peek().kind == *kind
    }

    fn advance(&mut self) -> &'tokens Token<'a> {
        if !self.is_at_end() { self.current += 1 }
        let prev_idx: usize = self.previous_index();
        self.tokens.index(prev_idx)
    }

    fn is_at_end(&self) -> bool { self.peek().kind == TokenKind::Eof }

    fn peek(&self) -> &'tokens Token<'a> { self.tokens.index(self.current) }

    fn previous_index(&self) -> usize { self.current - 1 }

    fn error(
        token: &'tokens Token<'a>,
        message: &'static str
    ) -> ParseError<'tokens, 'a> {
        ParseError { token, message }
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            let prev_token_idx: usize = self.previous_index();
            if self.tokens.index(prev_token_idx).kind == TokenKind::NewLine {
                return;
            }

            match self.peek().kind {
                TokenKind::Struct |
                TokenKind::Fn |
                TokenKind::Val |
                TokenKind::Var |
                TokenKind::Log |
                TokenKind::For |
                TokenKind::If |
                TokenKind::While |
                TokenKind::Return |
                TokenKind::Unsafe |
                TokenKind::Module |
                TokenKind::Import |
                TokenKind::Export => return,
                _ => { }
            }

            self.advance();
        }
    }
}