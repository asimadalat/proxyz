mod lexer;
mod driver;
mod parser;

use std::env;

use crate::driver::Proxyz;

fn main() {
    let args: Vec<String> = env::args().collect();
    Proxyz::execute(args);
}
