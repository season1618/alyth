mod data;
mod lexer;
mod parser;

use std::env;
use std::fs;

use crate::lexer::tokenize;
use crate::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_path = &args[1];

    let Ok(code) = fs::read_to_string(src_path) else {
        println!("could not open the source file.");
        return;
    };

    let tokens = tokenize(&code);
    println!("{:?}", tokens);
    let nodes = parse(tokens);
    println!("{:?}", nodes);
}
