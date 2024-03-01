mod data;
mod error;
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

    let code = &fs::read_to_string(src_path).expect("failed to open the source file");
    let mut dest = File::create(dest_path).expect("failed to create the destination file");

    compile(code, &mut dest);
}

fn compile(code: &str, dest: &mut File) {
    let tokens = match tokenize(code) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("token error: {}", e);
            return;
        },
    };
    let nodes = parse(tokens);
    gen_program(nodes, dest);
}

#[test]
fn test_expr() {
    test_compile("1  ", 1);
    test_compile(" 123", 123);
}

fn test_compile(code: &str, expected: i32) {
    use std::process::Command;

    let mut dest = fs::File::create("./test/main.s").unwrap();

    compile(code, &mut dest);

    Command::new("cc")
        .args(["-o", "test/main", "test/main.s"])
        .status()
        .expect("failed to execute process");

    let actual = Command::new(fs::canonicalize("./test/main").unwrap())
        .status()
        .expect("failed to execute process")
        .code().unwrap();
    
    if expected == actual {
        eprintln!("OK");
    } else {
        eprintln!("{}: expected {}, actual {}", code, expected, actual);
    }
}
