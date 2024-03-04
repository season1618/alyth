use std::fs::File;
use std::io::{self, Write};

use crate::data::{Defn, Stmt, Expr, BinOpKind, UnOpKind};

use Defn::*;
use Stmt::*;
use Expr::*;
use BinOpKind::*;
use UnOpKind::*;

pub fn gen_program<'a>(node: Defn<'a>, dest: &'a mut File) -> Result<(), io::Error> {
    let mut codegen = CodeGen::new(dest);
    codegen.gen_program(node)
}

struct CodeGen<'a> {
    dest: &'a mut File,
    label: u32,
}

impl<'a> CodeGen<'a> {
    fn new(dest: &'a mut File) -> Self {
        Self { dest, label: 0 }
    }

    fn gen_program(&'a mut self, defn: Defn<'a>) -> Result<(), io::Error> {
        writeln!(self.dest, ".intel_syntax noprefix")?;
        writeln!(self.dest, ".global main")?;
        self.gen_func_defn(defn)
    }

    fn gen_func_defn(&'a mut self, defn: Defn<'a>) -> Result<(), io::Error> {
        match defn {
            FuncDef { name, stmts } => {
                writeln!(self.dest, "{}:", name)?;
                for stmt in stmts {
                    self.gen_stmt(stmt)?;
                }
                writeln!(self.dest, "    ret")
            },
        }
    }

    fn gen_stmt(&mut self, stmt: Stmt) -> Result<(), io::Error> {
        match stmt {
            Stmt::Return(expr) => {
                self.gen_expr(expr)?;
                writeln!(self.dest, "    pop rax")?;
                writeln!(self.dest, "    ret")
            },
            ExprStmt(expr) => {
                self.gen_expr(expr)?;
                writeln!(self.dest, "    pop rax")
            },
        }
    }
    
    fn gen_expr(&mut self, expr: Expr) -> Result<(), io::Error> {
        match expr {
            BinOp { kind, lhs, rhs } => self.gen_binary(kind, *lhs, *rhs),
            UnOp { kind, operand } => self.gen_unary(kind, *operand),
            Num(val) => writeln!(self.dest, "    push {val}"),
        }
    }

    fn gen_binary(&mut self, kind: BinOpKind, lhs: Expr, rhs: Expr) -> Result<(), io::Error> {
        match kind {
            LogicOr => {
                let label = self.get_label();

                self.gen_expr(lhs)?;
                writeln!(self.dest, "    pop rax")?;
                writeln!(self.dest, "    cmp rax, 0")?;
                writeln!(self.dest, "    jne .L{label}")?;
                self.gen_expr(rhs)?;
                writeln!(self.dest, "    pop rax")?;
                writeln!(self.dest, ".L{label}:")?;
                return writeln!(self.dest, "    push rax");
            },
            LogicAnd => {
                let label = self.get_label();

                self.gen_expr(lhs)?;
                writeln!(self.dest, "    pop rax")?;
                writeln!(self.dest, "    cmp rax, 0")?;
                writeln!(self.dest, "    je .L{label}")?;
                self.gen_expr(rhs)?;
                writeln!(self.dest, "    pop rax")?;
                writeln!(self.dest, ".L{label}:")?;
                return writeln!(self.dest, "    push rax");
            },
            _ => {},
        }

        self.gen_expr(lhs)?;
        self.gen_expr(rhs)?;
        writeln!(self.dest, "    pop rdi")?;
        writeln!(self.dest, "    pop rax")?;
        match kind {
            Eq => {
                writeln!(self.dest, "    cmp rax, rdi")?;
                writeln!(self.dest, "    sete al")?;
                writeln!(self.dest, "    movzb rax, al")?;
            },
            Neq => {
                writeln!(self.dest, "    cmp rax, rdi")?;
                writeln!(self.dest, "    setne al")?;
                writeln!(self.dest, "    movzb rax, al")?;
            },
            Leq => {
                writeln!(self.dest, "    cmp rax, rdi")?;
                writeln!(self.dest, "    setle al")?;
                writeln!(self.dest, "    movzb rax, al")?;
            },
            Lt => {
                writeln!(self.dest, "    cmp rax, rdi")?;
                writeln!(self.dest, "    setl al")?;
                writeln!(self.dest, "    movzb rax, al")?;
            },
            Add => writeln!(self.dest, "    add rax, rdi")?,
            Sub => writeln!(self.dest, "    sub rax, rdi")?,
            Mul => writeln!(self.dest, "    imul rax, rdi")?,
            Div => {
                writeln!(self.dest, "    cqo")?;
                writeln!(self.dest, "    idiv rdi")?;
            },
            Mod => {
                writeln!(self.dest, "    cqo")?;
                writeln!(self.dest, "    idiv rdi")?;
                return writeln!(self.dest, "    push rdx");
            },
            _ => {},
        }
        writeln!(self.dest, "    push rax")
    }

    fn gen_unary(&mut self, kind: UnOpKind, operand: Expr) -> Result<(), io::Error> {
        self.gen_expr(operand)?;
        writeln!(self.dest, "    pop rax")?;
        match kind {
            LogicNot => writeln!(self.dest, "    xor rax, 1")?,
            Neg => writeln!(self.dest, "    neg rax")?,
        }
        writeln!(self.dest, "    push rax")
    }

    fn get_label(&mut self) -> u32 {
        let label = self.label;
        self.label += 1;
        label
    }
}
