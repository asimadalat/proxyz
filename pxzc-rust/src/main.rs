mod lexer;
mod proxyz;
mod parser;
mod errors;

use std::env;

use crate::proxyz::Proxyz;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut proxyz = Proxyz::new();
    proxyz.execute(args);
}
