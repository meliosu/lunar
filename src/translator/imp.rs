#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::fmt::{write, Write};

use chumsky::chain::Chain;

use crate::ast::visit::Visit;
use crate::ast::{self, ItemSub, Type};

macro_rules! error {
    ($self:ident, $($args:tt)*) => {
        $self.errors.push(anyhow::anyhow!($($args)*))
    };
}

macro_rules! emit {
    ($out:expr, $($args:tt)*) => {
        write!($out, $($args)*).unwrap()
    };
}

#[derive(Clone)]
struct ExternOp {
    params: Vec<ast::Param>,
}

#[derive(Clone)]
struct StructOp {
    params: Vec<ast::Param>,
}

pub struct Translator {
    errors: Vec<anyhow::Error>,
    externs: HashMap<String, ExternOp>,
    structs: HashMap<String, StructOp>,
    blocks: HashMap<String, Block>,
    block_id: u64,
}

#[derive(Clone)]
struct Block {
    local: HashSet<String>,
    wait: HashSet<String>,
    submit: HashSet<String>,
    ctx: HashMap<String, ast::Type>,
    id: u64,
    ty: BlockType,
}

#[derive(Clone)]
enum BlockType {
    Fork(BlockFork),
    For(BlockFor),
    If(BlockIf),
    ExternCall(BlockExternCall),
    SubCall(BlockSubCall),
}

#[derive(Clone)]
struct BlockFork {
    children: Vec<Block>,
    decls: Vec<String>,
}

#[derive(Clone)]
struct BlockFor {
    index: ast::Ident,
    lower: ast::Expr,
    upper: ast::Expr,
    block: Box<Block>,
}

#[derive(Clone)]
struct BlockIf {
    cond: ast::Expr,
    then: Box<Block>,
}

#[derive(Clone)]
struct BlockExternCall {
    symbol: ast::Ident,
    args: Vec<ast::Expr>,
    params: Vec<ast::Param>,
}

#[derive(Clone)]
struct BlockSubCall {
    symbol: ast::Ident,
    args: Vec<ast::Expr>,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            externs: HashMap::new(),
            structs: HashMap::new(),
            blocks: HashMap::new(),
            block_id: 0,
        }
    }

    pub fn translate(mut self, file: ast::File) -> String {
        self.visit_file(&file);

        let mut out = String::new();

        emit!(&mut out, "{}", include_str!("../../c/runtime.h"));

        for (name, ExternOp { params }) in self.externs.clone() {
            emit!(&mut out, "void {name}(");

            for (i, param) in params.iter().enumerate() {
                emit!(
                    &mut out,
                    "{} {}",
                    match param.ty {
                        ast::Type::Int => "int",
                        ast::Type::Float => "float",
                        ast::Type::Input => "df",
                        ast::Type::Output => "df",
                    },
                    param.name.name,
                );

                if i < params.len() - 1 {
                    emit!(&mut out, ",");
                }
            }

            emit!(&mut out, ");");
        }

        let main = self.blocks.get("main").unwrap();

        emit!(&mut out, "void *entry = block_{};", main.id);

        for (_, block) in self.blocks.clone() {
            self.generate_block(block, &mut out);
        }

        out
    }

    fn generate_block(&mut self, block: Block, out: &mut String) {
        let mut gen = BlockGenerator::new();
        gen.gen(&block);
        emit!(out, "{}", gen.finish());

        match block.ty {
            BlockType::Fork(BlockFork { children, decls }) => {
                for child in children {
                    self.generate_block(child, out);
                }
            }

            BlockType::For(block_for) => {
                self.generate_block(*block_for.block, out);
            }

            BlockType::If(block_if) => {
                self.generate_block(*block_if.then, out);
            }

            BlockType::ExternCall(block_extern_call) => {}
            BlockType::SubCall(block_sub_call) => {}
        }
    }

    fn id(&mut self) -> u64 {
        let id = self.block_id;
        self.block_id += 1;
        id
    }
}

impl<'ast> Visit<'ast> for Translator {
    fn visit_item_import(&mut self, import: &'ast ast::ItemImport) {
        self.externs.insert(
            import.signature.ident.name.clone(),
            ExternOp {
                params: import.signature.params.clone(),
            },
        );
    }

    fn visit_item_sub(&mut self, sub: &'ast ast::ItemSub) {
        self.structs.insert(
            sub.signature.ident.name.clone(),
            StructOp {
                params: sub.signature.params.clone(),
            },
        );

        let block = self.fold_sub(sub.clone());
        self.blocks.insert(sub.signature.ident.name.clone(), block);
    }
}

impl Translator {
    fn fold_sub(&mut self, sub: ast::ItemSub) -> Block {
        let mut block = self.fold_block(sub.body);

        for ast::Param { name, ty } in sub.signature.params {
            block.ctx.insert(name.name, ty);
        }

        block
    }

    fn fold_block(&mut self, block: ast::Block) -> Block {
        let mut local = HashSet::new();
        let mut children = Vec::new();
        let mut ctx = HashMap::new();
        let mut decls = Vec::new();

        for stmt in block.stmts {
            let child = match stmt {
                ast::Stmt::Decl(stmt_decl) => {
                    for df in stmt_decl.dfs {
                        local.insert(df.name.clone());
                        decls.push(df.name);
                    }

                    continue;
                }

                ast::Stmt::Block(stmt_block) => self.fold_stmt_block(stmt_block),
                ast::Stmt::For(stmt_for) => self.fold_stmt_for(stmt_for),
                ast::Stmt::If(stmt_if) => self.fold_stmt_if(stmt_if),
                ast::Stmt::Call(stmt_call) => self.fold_stmt_call(stmt_call),
            };

            for (name, ty) in child.ctx.clone() {
                ctx.insert(name, ty);
            }

            children.push(child);
        }

        Block {
            id: self.id(),
            local,
            wait: HashSet::new(),
            submit: HashSet::new(),
            ctx,
            ty: BlockType::Fork(BlockFork { children, decls }),
        }
    }

    fn fold_stmt_block(&mut self, stmt: ast::StmtBlock) -> Block {
        self.fold_block(stmt.block)
    }

    fn fold_stmt_for(&mut self, stmt: ast::StmtFor) -> Block {
        let mut wait = HashSet::new();
        let mut ctx = HashMap::new();
        let mut local = HashSet::new();
        local.insert(stmt.index.name.clone());

        let body = self.fold_block(stmt.body);

        for (name, ty) in body.ctx.clone() {
            ctx.insert(name, ty);
        }

        let captures: HashSet<_> = captures_expr(&stmt.lower)
            .union(&captures_expr(&stmt.upper))
            .cloned()
            .collect();

        for capture in captures {
            wait.insert(capture);
        }

        Block {
            id: self.id(),
            local,
            wait,
            submit: HashSet::new(),
            ctx,
            ty: BlockType::For(BlockFor {
                index: stmt.index.clone(),
                lower: stmt.lower.clone(),
                upper: stmt.upper.clone(),
                block: Box::new(body),
            }),
        }
    }

    fn fold_stmt_if(&mut self, stmt: ast::StmtIf) -> Block {
        let mut ctx = HashMap::new();
        let mut wait = HashSet::new();

        let then = self.fold_block(stmt.then);

        for (name, ty) in then.ctx.clone() {
            ctx.insert(name, ty);
        }

        let captures = captures_expr(&stmt.cond);

        for capture in captures {
            wait.insert(capture);
        }

        Block {
            id: self.id(),
            local: HashSet::new(),
            wait,
            submit: HashSet::new(),
            ctx,
            ty: BlockType::If(BlockIf {
                cond: stmt.cond.clone(),
                then: Box::new(then),
            }),
        }
    }

    fn fold_stmt_call(&mut self, stmt: ast::StmtCall) -> Block {
        let extern_op = self.externs.get(&stmt.ident.name).unwrap();

        let mut wait = HashSet::new();
        let mut submit = HashSet::new();
        let mut ctx = HashMap::new();
        let mut params = Vec::new();

        for (param, arg) in extern_op.params.iter().zip(&stmt.args) {
            match &param.ty {
                ast::Type::Int | ast::Type::Float => {
                    for capture in captures_expr(arg) {
                        ctx.insert(capture.clone(), param.ty.clone());
                        wait.insert(capture);
                    }
                }

                ast::Type::Input => {
                    for capture in captures_expr(arg) {
                        ctx.insert(capture.clone(), param.ty.clone());
                        wait.insert(capture);
                    }
                }

                ast::Type::Output => {
                    for capture in captures_expr(arg) {
                        ctx.insert(capture.clone(), param.ty.clone());
                        submit.insert(capture);
                    }
                }
            }

            params.push(param.clone());
        }

        Block {
            id: self.id(),
            local: HashSet::new(),
            wait,
            submit,
            ctx,
            ty: BlockType::ExternCall(BlockExternCall {
                symbol: stmt.ident.clone(),
                args: stmt.args.clone(),
                params,
            }),
        }
    }
}

struct BlockGenerator {
    out: String,
}

impl BlockGenerator {
    fn new() -> Self {
        Self { out: String::new() }
    }

    fn finish(self) -> String {
        self.out
    }

    fn gen(&mut self, block: &Block) {
        self.gen_ctx(block);
        self.gen_signature(block);
        self.gen_body(block);
    }

    fn gen_body(&mut self, block: &Block) {
        emit!(&mut self.out, "{{");

        self.gen_requests(block);

        match &block.ty {
            BlockType::Fork(block_fork) => {
                for df in &block_fork.decls {
                    emit!(&mut self.out, "df {df}=df_create();");
                }

                for child in &block_fork.children {
                    self.gen_fork(block, child);
                }
            }

            BlockType::For(block_for) => {
                let index_name = &block_for.index.name;

                emit!(&mut self.out, "for (int {}=", index_name);
                self.gen_expr(block, &block_for.lower, Type::Int);
                emit!(&mut self.out, "; {}<", index_name);
                self.gen_expr(block, &block_for.upper, Type::Int);
                emit!(&mut self.out, "; {}++)", index_name);
                self.gen_fork(block, &block_for.block);
            }

            BlockType::If(block_if) => {
                emit!(&mut self.out, "if (");
                self.gen_expr(block, &block_if.cond, Type::Int);
                emit!(&mut self.out, ")");
                self.gen_fork(block, &block_if.then);
            }

            BlockType::ExternCall(call) => {
                emit!(&mut self.out, "{}(", call.symbol.name);

                for ((i, arg), param) in call.args.iter().enumerate().zip(&call.params) {
                    self.gen_expr(block, &arg, param.ty.clone());

                    if i < call.args.len() - 1 {
                        emit!(&mut self.out, ",");
                    }
                }

                emit!(&mut self.out, ");");
            }

            BlockType::SubCall(block_sub_call) => {
                todo!()
            }
        }

        self.gen_submits(block);

        emit!(&mut self.out, "dealloc(ctx);");
        emit!(&mut self.out, "return EXIT;");
        emit!(&mut self.out, "}}");
    }

    fn gen_requests(&mut self, block: &Block) {
        for name in &block.wait {
            emit!(
                &mut self.out,
                "if (request(&ctx->{name})) {{ return WAIT; }}"
            )
        }
    }

    fn gen_submits(&mut self, block: &Block) {
        for name in &block.submit {
            emit!(&mut self.out, "submit(ctx->{name});")
        }
    }

    fn gen_signature(&mut self, block: &Block) {
        emit!(
            &mut self.out,
            "Action block_{}(block_ctx_{} *ctx)",
            block.id,
            block.id
        );
    }

    fn gen_ctx(&mut self, block: &Block) {
        emit!(&mut self.out, "typedef struct {{");

        for (name, ty) in &block.ctx {
            let ty = match ty {
                ast::Type::Int => "int",
                ast::Type::Float => "float",
                ast::Type::Input | ast::Type::Output => "df",
            };

            emit!(&mut self.out, "{ty} {name};",);
        }

        emit!(&mut self.out, "}} block_ctx_{};", block.id);
    }

    fn gen_fork(&mut self, parent: &Block, child: &Block) {
        emit!(&mut self.out, "{{");
        emit!(
            &mut self.out,
            "block_ctx_{} *child_ctx = alloc(sizeof(*child_ctx));",
            child.id
        );

        for (name, _) in &child.ctx {
            if parent.local.contains(name) {
                emit!(&mut self.out, "child_ctx->{name}={name};");
            } else {
                emit!(&mut self.out, "child_ctx->{name}=ctx->{name};");
            }
        }

        emit!(&mut self.out, "spawn(block_{}, child_ctx);", child.id);
        emit!(&mut self.out, "}}");
    }

    fn gen_expr(&mut self, block: &Block, expr: &ast::Expr, ty: Type) {
        match expr {
            ast::Expr::Ident(ident) => {
                let name = &ident.ident.name;

                // ???
                if block.local.contains(name) {
                    emit!(&mut self.out, "{name}");
                } else if block.wait.contains(name) {
                    match ty {
                        Type::Input | Type::Output => {
                            emit!(&mut self.out, "wait(ctx->{name})");
                        }

                        _ => {
                            emit!(&mut self.out, "cast_int(wait(ctx->{name}))");
                        }
                    }
                } else {
                    emit!(&mut self.out, "ctx->{name}");
                }
            }

            ast::Expr::Lit(lit) => match &lit.lit {
                ast::Lit::Int(int) => {
                    emit!(&mut self.out, "{}", int.value);
                }

                ast::Lit::Float(float) => {
                    emit!(&mut self.out, "{}", float.value);
                }
            },

            ast::Expr::BinOp(binop) => {
                self.gen_expr(block, &binop.lhs, ty.clone());

                let op = match &binop.op {
                    ast::Op::Add => "+",
                    ast::Op::Sub => "-",
                    ast::Op::Mul => "*",
                    ast::Op::Div => "/",
                };

                emit!(&mut self.out, "{op}");

                self.gen_expr(block, &binop.rhs, ty);
            }
        }
    }
}

fn captures_expr(expr: &ast::Expr) -> HashSet<String> {
    match expr {
        ast::Expr::Ident(expr_ident) => {
            let mut set = HashSet::new();
            set.insert(expr_ident.ident.name.clone());
            set
        }

        ast::Expr::BinOp(binop) => {
            let mut set = HashSet::new();

            for capture in captures_expr(&binop.lhs) {
                set.insert(capture);
            }

            for capture in captures_expr(&binop.rhs) {
                set.insert(capture);
            }

            set
        }

        _ => HashSet::new(),
    }
}

pub struct Captures {
    idents: HashSet<String>,
}

impl Captures {
    fn new() -> Self {
        Self {
            idents: HashSet::new(),
        }
    }

    fn finish(self) -> HashSet<String> {
        self.idents
    }
}

impl<'ast> Visit<'ast> for Captures {
    fn visit_expr_ident(&mut self, ident: &'ast ast::ExprIdent) {
        self.idents.insert(ident.ident.name.clone());
    }
}
