use std::fs;
use std::io;
use std::process;
use crate::lexer::{Scanner, Token, TokenKind};
use crate::parser;
use crate::parser::core::Parser;

static HAD_ERROR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub struct Proxyz;

impl Proxyz {
    pub fn execute(args: Vec<String>) {
        let num_args: u8 = args.len() as u8;

        if num_args > 2 {
            println!("Usage: pxzc [script]");
            process::exit(1);
        } else if num_args == 2 {
            Proxyz::run_file(&args[1]).expect("Could not run Proxyz script");
        } else {
            Proxyz::run_prompt();
        }
    }

    fn run_file(path: &String) -> io::Result<()> {
        let s = fs::read_to_string(&path)?;

        Proxyz::run(&s);
        if HAD_ERROR.load(std::sync::atomic::Ordering::Relaxed) {
            process::exit(1)
        }

        Ok(())
    }

    fn run_prompt() {
        let input = io::stdin();
        let mut buffer = String::new();
        loop {
            print!("> ");
            let line = input.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    Proxyz::run(&buffer);
                    buffer.clear();
                    HAD_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
                }
                Err(ex) => eprintln!("error: {ex}"),
            }
        }
    }

    fn run(source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Some(expression) => {
                println!("{}", parser::ast_printer::print(&expression));
            }
            None => { }
        }
    }

    pub fn error_at_line(line: u32, message: &str) { Self::report_error(line, "", message) }

    pub fn error_at_token(token: &Token, message: &str) {
        if token.kind == TokenKind::Eof {
            Self::report_error(token.line, " at end", message);
        } else {
            Self::report_error(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn report_error(line: u32, r#where: &str, message: &str) {
        eprintln!("[line {line}] Error {where} : {message}");
        HAD_ERROR.store(true, std::sync::atomic::Ordering::Relaxed)
    }
}