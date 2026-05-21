use std::fs;
use std::io;
use std::process;
use crate::lexer::{Scanner, Token, TokenKind};
use crate::parser;
use crate::parser::core::Parser;

pub struct Proxyz {
    had_error: bool,
}

impl Proxyz {
    pub fn new() -> Proxyz {
        Proxyz { had_error: false }
    }

    pub fn execute(&mut self, args: Vec<String>) {
        let num_args: u8 = args.len() as u8;

        if num_args > 2 {
            println!("Usage: pxzc [script]");
            process::exit(1);
        } else if num_args == 2 {
            self.run_file(&args[1]).expect("Could not run Proxyz script");
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, path: &String) -> io::Result<()> {
        let s = fs::read_to_string(&path)?;

        self.run(&s);
        if self.had_error {
            process::exit(1)
        }

        Ok(())
    }

    fn run_prompt(&mut self) {
        let input = io::stdin();
        let mut buffer = String::new();
        loop {
            print!("> ");
            let line = input.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    self.run(&buffer);
                    buffer.clear();
                    self.had_error = true;
                }
                Err(ex) => eprintln!("error: {ex}"),
            }
        }
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);

        match scanner.scan_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);

                match parser.parse() {
                    Ok(ast) => {
                        println!("{}", parser::ast_printer::print(&ast));
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