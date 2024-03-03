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
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("token error: {}", e);
            return;
        },
    };
    let nodes = match parse(tokens) {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("parse error: {}", e);
            return ;
        },
    };
    match gen_program(nodes, dest) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("output error: {}", e);
            return;
        },
    }
}

#[test]
fn test_expr() {
    test_compile("1  ", 1);
    test_compile(" -1", 255);
    test_compile(" 123", 123);
    test_compile("1+2+3", 6);
    test_compile("1+2-3", 0);
    test_compile("1--1", 2);
    test_compile("2*5/3", 3);
    test_compile("2*6/3", 4);
    test_compile("2*10%7", 6);
    test_compile("6-3/2", 5);
    test_compile("1+2+3+4+5+6+7+8*9", 100);
    test_compile("-12/3+4+3%2", 1);
    test_compile("(1+2)*(-3+5)/(10-7)", 2);
    test_compile("1+4==2+3", 1);
    test_compile("1+4!=2+3", 0);
    test_compile("1+4<=2+3", 1);
    test_compile("1+4 <2+3", 0);
    test_compile("1+4>=2+3", 1);
    test_compile("1+4 >2+3", 0);
    test_compile("!0", 1);
    test_compile("!1", 0);
    test_compile("0 || 0", 0);
    test_compile("0 || 1", 1);
    test_compile("1 || 0", 1);
    test_compile("1 || 1", 1);
    test_compile("0 && 0", 0);
    test_compile("0 && 1", 0);
    test_compile("1 && 0", 0);
    test_compile("1 && 1", 1);
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
