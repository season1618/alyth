use std::fs::File;
use std::io::{self, Write};

use crate::data::{Expr, BinOpKind, UnOpKind};

use Expr::*;
use BinOpKind::*;
use UnOpKind::*;

pub fn gen_program<'a>(node: Expr, dest: &'a mut File) -> Result<(), io::Error> {
    let mut codegen = CodeGen::new(dest);
    codegen.gen_program(node)
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
            BinOp { kind, lhs, rhs } => self.gen_binary(kind, *lhs, *rhs),
            UnOp { kind, operand } => self.gen_unary(kind, *operand),
            Num(val) => writeln!(self.dest, "    push {val}"),
        }
    }

    fn gen_binary(&mut self, kind: BinOpKind, lhs: Expr, rhs: Expr) -> Result<(), io::Error> {
        self.gen_expr(lhs)?;
        self.gen_expr(rhs)?;
        writeln!(self.dest, "    pop rdi")?;
        writeln!(self.dest, "    pop rax")?;
        match kind {
            Add => writeln!(self.dest, "    add rax, rdi")?,
            Sub => writeln!(self.dest, "    sub rax, rdi")?,
            Mul => writeln!(self.dest, "    imul rax, rdi")?,
            Div => {
                writeln!(self.dest, "    cqo");
                writeln!(self.dest, "    idiv rdi");
            },
            Mod => {
                writeln!(self.dest, "    cqo");
                writeln!(self.dest, "    idiv rdi");
                return writeln!(self.dest, "    push rdx");
            },
        }
        writeln!(self.dest, "    push rax")
    }

    fn gen_unary(&mut self, kind: UnOpKind, operand: Expr) -> Result<(), io::Error> {
        self.gen_expr(operand)?;
        writeln!(self.dest, "    pop rax")?;
        match kind {
            Neg => writeln!(self.dest, "    neg rax")?,
        }
        writeln!(self.dest, "    push rax")
    }
}
