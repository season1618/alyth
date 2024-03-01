mod data;
mod lexer;
mod parser;
mod codegen;

use std::env;
use std::fs::{self, File};

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::codegen::gen_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_path = &args[1];
    let dest_path = &args[2];

    let code = fs::read_to_string(src_path).expect("failed to open the source file");
    let mut dest = File::create(dest_path).expect("failed to create the destination file");

    let tokens = tokenize(&code);
    eprintln!("{:?}", tokens);
    let nodes = parse(tokens);
    eprintln!("{:?}", nodes);
    gen_program(nodes, &mut dest);
}
