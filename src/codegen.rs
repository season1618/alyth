use crate::data::*;

use Expr::*;

pub fn gen_program(expr: Expr) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    gen_expr(expr);
    println!("    pop rax");
    println!("    ret");
}

fn gen_expr(expr: Expr) {
    match expr {
        Num(val) => { println!("    push {val}"); },
        _ => {},
    }
}