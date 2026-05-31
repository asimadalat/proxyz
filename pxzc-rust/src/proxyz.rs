use std::io::Write;

use crate::interpreter::core::Interpreter;
use crate::lexer::{Scanner, Token, TokenKind};
use crate::parser::core::Parser;

pub(crate) struct Proxyz<'a> {
    interpreter: Interpreter<'a>,
    pub(crate) had_error: bool,
}

impl<'a> Proxyz<'a> {
    pub(crate) fn new() -> Proxyz<'a> {
        Proxyz {
            interpreter: Interpreter::new(),
            had_error: false
        }
    }

    pub(crate) fn run_prompt(&mut self) {
        let input = std::io::stdin();

        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();

            let mut buffer = String::new();
            if input.read_line(&mut buffer).is_err() || buffer.trim() == "exit" {
                break;
            }

            let line_static: &'static str = Box::leak(buffer.into_boxed_str());
            self.run(line_static);
        }
    }

    pub(crate) fn run(&mut self, source: &'a str) {
        let mut scanner = Scanner::new(source);

        match scanner.scan_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);

                match parser.parse() {
                    Ok(ast) => {
                        match self.interpreter.interpret(ast) {
                            Ok(_) => {}
                            Err(runtime_error) => {
                                self.error_at_token(
                                    &runtime_error.token,
                                    &*runtime_error.message
                                )
                            }
                        }

                        // println!("{}", parser::ast_printer::print(&ast));
                    }
                    Err(parse_error) => { 
                        self.error_at_token(
                            parse_error.token, 
                            parse_error.message
                        )
                    }
                }
            }
            Err(scan_error) => {
                self.error_at_line(scan_error.line, scan_error.message)
            }
        }
    }

    fn error_at_line(&mut self, line: u32, message: &str) {
        self.report_error(line, "", message)
    }

    fn error_at_token(&mut self, token: &Token, message: &str) {
        if token.kind == TokenKind::Eof {
            self.report_error(token.line, " at end", message);
        } else {
            self.report_error(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn report_error(&mut self, line: u32, r#where: &str, message: &str) {
        eprintln!("[line {line}] Error {where} : {message}");
        self.had_error = true;
    }
}