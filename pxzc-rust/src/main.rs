mod lexer;
mod proxyz;
mod parser;
mod errors;
mod interpreter;

use crate::proxyz::Proxyz;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num_args: u8 = args.len() as u8;

    if num_args > 2 {
        println!("Usage: pxzc [script]");
        std::process::exit(1);
    }

    let mut proxyz = Proxyz::new();

    if num_args == 2 {
        let source = std::fs::read_to_string(&args[1])
            .expect("Could not read Proxyz file.");

        proxyz.run(&source);
        if proxyz.had_error {
            std::process::exit(1);
        }
    } else { proxyz.run_prompt(); }
}
