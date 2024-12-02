#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    hash::Hash,
};

use chumsky::chain::Chain;

use crate::ast::{self, visit::Visit, StmtIf};

macro_rules! emit {
    ($out:expr, $($args:tt)*) => {
        write!($out, $($args)*).unwrap()
    };
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Int,
    Float,
    Id,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    cx: HashMap<String, Type>,
    ty: BlockType,
    local: HashSet<String>,
}

#[derive(Debug, Clone)]
enum BlockType {
    Fork(BlockFork),
    For(BlockFor),
    If(BlockIf),
    ExternCall(BlockExternCall),
}

#[derive(Debug, Clone)]
struct BlockFork {
    decls: HashSet<String>,
    children: Vec<Block>,
}

#[derive(Debug, Clone)]
struct BlockFor {
    index: String,
    lower: ast::Expr,
    upper: ast::Expr,
    block: Box<Block>,
}

#[derive(Debug, Clone)]
struct BlockIf {
    cond: ast::Expr,
    then: Box<Block>,
}

#[derive(Debug, Clone)]
struct BlockExternCall {
    call: ast::StmtCall,
    wait: Vec<String>,
    submit: Vec<String>,
}

struct Generator;

impl Generator {
    fn gen_block(&self, block: &Block, out: &mut String) {
        self.gen_block_cx(block, out);
        self.gen_block_signature(block, out);
        self.gen_block_body(block, out);
    }

    fn gen_block_body(&self, block: &Block, out: &mut String) {
        emit!(out, "{{");
        self.gen_block_prelude(block, out);

        match &block.ty {
            BlockType::Fork(bfork) => {
                for decl in &bfork.decls {
                    emit!(out, "ID {} = alloc_id(self);", decl);
                }

                for child in &bfork.children {
                    self.gen_fork(child, out);
                }
            }

            BlockType::For(bfor) => {
                let index = &bfor.index;

                let mut lower = String::new();
                self.gen_expr(&bfor.lower, &mut lower);

                let mut upper = String::new();
                self.gen_expr(&bfor.upper, &mut upper);

                emit!(out, "for (int {index}={lower}; {index}<{upper}; {index}++)",);

                self.gen_fork(&bfor.block, out);
            }

            BlockType::If(bif) => {
                let mut cond = String::new();
                self.gen_expr(&bif.cond, &mut cond);

                emit!(out, "if ({cond})");
                self.gen_fork(&bif.then, out);
            }

            BlockType::ExternCall(BlockExternCall { call, submit, wait }) => {
                for df in wait {
                    emit!(out, "if (request(self, cx->{})) {{ return WAIT; }}", df);
                }

                emit!(out, "{}(", call.ident.name);

                for (i, arg) in call.args.iter().enumerate() {
                    emit!(out, "arg");

                    if i < call.args.len() - 1 {
                        emit!(out, ",");
                    }
                }

                emit!(out, ");");

                for df in submit {
                    emit!(out, "submit(self, cx->{});", df);
                }
            }
        }

        emit!(out, "free(cx);");
        emit!(out, "return EXIT;");
        emit!(out, "}}");
    }

    fn gen_fork(&self, block: &Block, out: &mut String) {
        emit!(out, "{{");
        emit!(
            out,
            "struct block_cx_{} *child = malloc(sizeof(*child));",
            block.id
        );

        for (name, _) in &block.cx {
            if !block.local.contains(name) {
                emit!(out, "child->{name} = cx->{name};");
            } else {
                emit!(out, "child->{name} = {name}");
            }
        }

        emit!(out, "spawn(self, block_{}, child);", block.id);
        emit!(out, "}}");
    }

    fn gen_block_cx(&self, block: &Block, out: &mut String) {
        emit!(out, "struct block_cx_{} {{", block.id);

        for (name, ty) in &block.cx {
            let ty = match ty {
                Type::Int => "int",
                Type::Float => "float",
                Type::Id => "ID",
            };

            emit!(out, "{ty} {name};");
        }

        emit!(out, "}};");
    }

    fn gen_block_signature(&self, block: &Block, out: &mut String) {
        emit!(out, "Status block_{}(CF *self)", block.id);
    }

    fn gen_block_prelude(&self, block: &Block, out: &mut String) {
        emit!(out, "struct block_cx_{} *cx = self->cx;", block.id);
    }

    fn gen_expr(&self, expr: &ast::Expr, out: &mut String) {
        match expr {
            ast::Expr::Ident(ident) => {
                emit!(out, "wait(self, cx->{})", ident.ident.name);
            }

            ast::Expr::Lit(lit) => match &lit.lit {
                ast::Lit::Int(int) => emit!(out, "{}", int.value),
                ast::Lit::Float(float) => emit!(out, "{}", float.value),
            },

            ast::Expr::BinOp(binop) => {
                self.gen_expr(&binop.lhs, out);

                let sign = match &binop.op {
                    ast::Op::Add => "+",
                    ast::Op::Sub => "-",
                    ast::Op::Mul => "*",
                    ast::Op::Div => "/",
                };

                emit!(out, "{sign}");

                self.gen_expr(&binop.rhs, out);
            }
        }
    }
}
