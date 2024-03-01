use std::fs::File;
use std::io::{self, Write};

use crate::data::*;

use Expr::*;

pub fn gen_program<'a>(node: Expr, dest: &'a mut File) {
    let mut codegen = CodeGen::new(dest);
    codegen.gen_program(node);
}

struct CodeGen<'a> {
    dest: &'a mut File,
}

impl<'a> CodeGen<'a> {
    fn new(dest: &'a mut File) -> Self {
        Self { dest }
    }

    fn gen_program(&'a mut self, expr: Expr) -> Result<(), io::Error> {
        writeln!(self.dest, ".intel_syntax noprefix")?;
        writeln!(self.dest, ".global main")?;
        writeln!(self.dest, "main:")?;
        self.gen_expr(expr)?;
        writeln!(self.dest, "    pop rax")?;
        writeln!(self.dest, "    ret")
    }
    
    fn gen_expr(&mut self, expr: Expr) -> Result<(), io::Error> {
        match expr {
            Num(val) => writeln!(self.dest, "    push {val}"),
        }
    }
}
