use crate::errors::{RuntimeError, RuntimeResult};
use crate::lexer::Token;
use std::collections::HashMap;
use crate::interpreter::core::RuntimeValue;

pub(crate) struct Environment<'a> {
    variables: HashMap<&'a str, RuntimeValue<'a>>,
}

impl<'a> Environment<'a> {
    pub(crate) fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }
    
    pub(crate) fn define(&mut self, name: &'a str, value: RuntimeValue<'a>) {
        self.variables.insert(name, value);
    }

    pub(crate) fn get(&self, name: Token<'a>) -> RuntimeResult<'a, RuntimeValue<'a>> {
        match self.variables.get(name.lexeme) {
            Some(value) => Ok(*value),
            None => Err(RuntimeError::new_owned(
                name,
                format!("Attempted to access undefined variable, {}.", name.lexeme)
            ))
        }
    }

    pub(crate) fn assign(
        &mut self,
        name: Token<'a>,
        value: RuntimeValue<'a>
    ) -> RuntimeResult<'a, ()> {
        if let Some(current) = self.variables.get_mut(name.lexeme) {
            *current = value;
            return Ok(());
        }

        Err(RuntimeError::new_owned(
            name,
            format!("Attempted to reassign undefined variable, {}.", name.lexeme)
        ))
    }
}
