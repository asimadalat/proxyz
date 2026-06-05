use crate::errors::{RuntimeError, RuntimeResult};
use crate::interpreter::environment::Environment;
use crate::lexer::{Literal, Token, TokenKind};
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;

#[derive(Debug, Clone, Copy)]
pub(crate) enum RuntimeValue<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    None
}

pub(crate) struct Interpreter<'a> {
    env: Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub(crate) fn new() -> Self {
        Self {
            env: Environment::new()
        }
    }

    pub(crate) fn interpret(
        &mut self,
        statements: Vec<Stmt<'a>>
    ) -> RuntimeResult<'a, ()> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, stmt: Stmt<'a>) -> RuntimeResult<'a, ()> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(*expr)?;
            },
            Stmt::Log(expr) => {
                let value = self.evaluate(*expr)?;
                match value {
                    RuntimeValue::Number(n) => {
                        println!("{}", n);
                    }
                    RuntimeValue::String(s) => {
                        println!("{}", s);
                    }
                    RuntimeValue::Boolean(b) => {
                        if b { print!("true"); } else { print!("false"); }
                    }
                    RuntimeValue::None => {
                        println!("None");
                    }
                }
            },
            Stmt::Var(name, initialiser) => {
                let mut value = RuntimeValue::None;
                if let Some(init) = initialiser {
                    value = self.evaluate(*init)?;
                }
                self.env.define(name.lexeme, value)
            }
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: Expr<'a>) -> RuntimeResult<'a, RuntimeValue<'a>> {
        match expr {
            Expr::Literal { value} => match value {
                Literal::Number(n) => Ok(RuntimeValue::Number(n)),
                Literal::String(s) => Ok(RuntimeValue::String(s)),
                Literal::Boolean(b) => Ok(RuntimeValue::Boolean(b)),
                Literal::None => Ok(RuntimeValue::None)
            },
            Expr::Grouped { expression } => self.evaluate(*expression),
            Expr::Unary { operator, operand } => {
                let value = self.evaluate(*operand)?;

                match operator.kind {
                    TokenKind::Minus => match value {
                        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(-n)),
                        _ => Err(Self::error(&operator, "Operand must be a number."))
                    },
                    TokenKind::Exclamation => match value {
                        RuntimeValue::Boolean(b) => Ok(RuntimeValue::Boolean(!b)),
                        _ => Err(Self::error(&operator, "Operand must be a boolean."))
                    }
                    _ => Err(Self::error(&operator, "Unknown unary operator."))
                }
            }
            Expr::Binary { left, operator, right } => {
                let left_value = self.evaluate(*left)?;
                let right_value = self.evaluate(*right)?;

                match operator.kind {
                    TokenKind::Plus => match (left_value, right_value) {
                        (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l + r)),
                        _ => Err(Self::error(&operator, "Operands must be numbers."))
                    }
                    TokenKind::Minus => match (left_value, right_value) {
                        (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l - r)),
                        _ => Err(Self::error(&operator, "Operands must be numbers."))
                    }
                    TokenKind::Star => match (left_value, right_value) {
                        (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l * r)),
                        _ => Err(Self::error(&operator, "Operands must be numbers."))
                    }
                    TokenKind::Slash => match (left_value, right_value) {
                        (RuntimeValue::Number(l), RuntimeValue::Number(r)) => {
                            if r == 0.0 {
                                return Err(Self::error(
                                    &operator,
                                    "Division by zero."
                                ));
                            }
                            Ok(RuntimeValue::Number(l / r))
                        },
                        _ => Err(Self::error(&operator, "Operands must be numbers."))
                    },
                    _ => Err(Self::error(&operator, "Unknown binary operator."))
                }
            }
            Expr::Variable { name } => { self.env.get(name) }
            Expr::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                self.env.assign(name, value)?;
                Ok(value)
            }
        }
    }

    fn error(token: &Token<'a>, message: &'static str) -> RuntimeError<'a> {
        RuntimeError::new(*token, message)
    }
}