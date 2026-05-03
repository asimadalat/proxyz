use std::fs;
use std::io;
use std::io::Read;
use std::process;

use crate::lexer::{Scanner, Token};

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

        for token in tokens {
            println!("{:?}\n", *token);
        }

        println!("{}", source);
    }

    pub fn exception(line: usize, message: &str) {
        eprintln!("[line {line}] Exception: {message}");
        HAD_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}