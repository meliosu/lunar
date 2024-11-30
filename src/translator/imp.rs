#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use std::fmt::Write;

use anyhow::bail;
use chumsky::chain::Chain;

use crate::ast::visit::Visit;
use crate::ast::{visit, Condition, Expr, Relation};
use crate::{ast, id};

type Code = String;

macro_rules! error {
    ($self:ident, $($args:tt)*) => {
        $self.errors.push(anyhow::anyhow!($($args)*));
        return;
    };
}

pub struct Translator {
    errors: Vec<anyhow::Error>,
    subs: HashMap<String, Sub>,
    imports: HashMap<String, Import>,
    block_id_gen: id::Generator,
    blocks: HashMap<String, Block>,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            subs: HashMap::new(),
            imports: HashMap::new(),
            block_id_gen: id::Generator::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn codegen(mut self, file: &ast::File) -> anyhow::Result<Code> {
        self.visit_file(file);
        self.finish()
    }

    fn finish(self) -> anyhow::Result<Code> {
        if !self.errors.is_empty() {
            let error: String = self
                .errors
                .into_iter()
                .map(|e| format!("error: {e}\n"))
                .collect();

            bail!("{error}");
        }

        let mut out = Code::new();

        write!(&mut out, "{}", include_str!("../../c/runtime.h"))?;

        for (name, import) in &self.imports {
            write!(&mut out, "void {name}(")?;

            for (i, param) in import.params.iter().enumerate() {
                match param.ty {
                    Type::Int => write!(&mut out, "int")?,
                    Type::Long => write!(&mut out, "long")?,
                    Type::Float => write!(&mut out, "float")?,
                    Type::Double => write!(&mut out, "double")?,
                    Type::Value => write!(&mut out, "DF*")?,
                    Type::Name => write!(&mut out, "DF*")?,
                }

                if i < import.params.len() - 1 {
                    write!(&mut out, ",")?;
                }
            }

            write!(&mut out, ");")?;
        }

        for (_, block) in self.blocks {
            write!(&mut out, "{}", block.code()?)?;
        }

        Ok(out)
    }

    fn transform_params(&self, params: Vec<ast::Param>) -> Vec<Param> {
        let mut counter = 0;

        params
            .into_iter()
            .map(|p| Param {
                name: p.ident.map(|i| i.ident).unwrap_or_else(|| {
                    counter += 1;
                    format!("unnamed_param_{}", counter)
                }),
                ty: match p.ty {
                    ast::Type::Int => Type::Int,
                    ast::Type::Long => Type::Long,
                    ast::Type::Float => Type::Float,
                    ast::Type::Double => Type::Double,
                    ast::Type::Value => Type::Value,
                    ast::Type::Name => Type::Name,
                },
            })
            .collect()
    }
}

impl visit::Visit for Translator {
    fn visit_import(&mut self, import: &ast::ItemImport) {
        let ident = if let Some(ref ident) = import.alias {
            ident
        } else {
            &import.signature.ident
        };

        self.imports.insert(
            ident.ident.clone(),
            Import {
                params: self.transform_params(import.signature.params.clone()),
            },
        );

        visit::visit_import(self, import);
    }

    fn visit_sub(&mut self, sub: &ast::ItemSub) {
        let ident = &sub.signature.ident;

        self.subs.insert(
            ident.ident.clone(),
            Sub {
                params: self.transform_params(sub.signature.params.clone()),
            },
        );

        let mut translator = BlockTranslator::new(self);

        match translator.fold_sub(sub) {
            Ok(block) => {
                self.blocks.insert(ident.ident.clone(), block);
            }

            Err(e) => {
                self.errors.push(e);
            }
        }

        visit::visit_sub(self, sub);
    }
}

#[derive(Clone)]
struct Sub {
    params: Vec<Param>,
}

#[derive(Clone)]
struct Import {
    params: Vec<Param>,
}

#[derive(Clone)]
enum Type {
    Int,
    Long,
    Float,
    Double,
    Value,
    Name,
}

#[derive(Clone)]
struct Param {
    name: String,
    ty: Type,
}

#[derive(Clone)]
struct Block {
    id: u64,
    dependencies: HashMap<String, Type>,
    kind: BlockKind,
}

#[derive(Clone)]
enum BlockKind {
    Fork(BlockFork),
    For(BlockFor),
    If(BlockIf),
    Call(BlockCall),
}

#[derive(Clone)]
struct BlockFork {
    children: Vec<Block>,
}

#[derive(Clone)]
struct BlockFor {
    index: String,
    lower: ast::Expr,
    upper: ast::Expr,
    child: Box<Block>,
}

#[derive(Clone)]
struct BlockIf {
    cond: ast::Condition,
    then: Box<Block>,
    else_: Option<Box<Block>>,
}

#[derive(Clone)]
struct BlockCall {
    call: ast::Call,
}

#[derive(Clone)]
struct BlockTranslator<'a> {
    translator: &'a Translator,
    types: HashMap<String, Type>,
}

impl Block {
    fn args_struct(&self) -> anyhow::Result<Code> {
        let mut out = String::new();

        write!(&mut out, "typedef struct {{")?;

        for (name, ty) in &self.dependencies {
            match ty {
                Type::Int => write!(&mut out, "int ")?,
                Type::Long => write!(&mut out, "long ")?,
                Type::Float => write!(&mut out, "float ")?,
                Type::Double => write!(&mut out, "double ")?,
                Type::Value => write!(&mut out, "DF ")?,
                Type::Name => write!(&mut out, "DF ")?,
            }

            write!(&mut out, "{name};")?;
        }

        write!(&mut out, "}} block_{}_context;", self.id)?;

        Ok(out)
    }

    fn signature(&self) -> anyhow::Result<Code> {
        let mut out = Code::new();

        write!(&mut out, "RET block_{}(CF *self)", self.id)?;

        Ok(out)
    }

    fn prelude(&self) -> anyhow::Result<Code> {
        let mut out = Code::new();

        write!(
            &mut out,
            "block_{}_context *context = self->context;",
            self.id
        )?;

        Ok(out)
    }

    fn code(&self) -> anyhow::Result<Code> {
        let mut out = Code::new();

        let mut children = Vec::new();

        write!(&mut out, "{}", self.signature()?)?;
        write!(&mut out, "{{")?;
        write!(&mut out, "{}", self.prelude()?)?;

        for (name, ty) in &self.dependencies {
            match ty {
                Type::Value => {
                    write!(
                        &mut out,
                        "if (request(self, &context->{name})) {{ return WAIT; }}"
                    )?;
                }

                _ => {}
            }
        }

        match self.kind {
            BlockKind::Fork(ref block_fork) => {
                for child in &block_fork.children {
                    children.push(child.clone());

                    write!(&mut out, "{{")?;

                    write!(
                        &mut out,
                        "block_{}_context *child = malloc(sizeof(*child));",
                        child.id
                    )?;

                    write!(&mut out, "spawn(self, block_{}, child)", child.id)?;

                    write!(&mut out, "}}")?;
                }
            }

            BlockKind::For(ref block_for) => {
                children.push(*block_for.child.clone());

                write!(
                    &mut out,
                    "for (int {index}={lower}; i<{upper}; {index}++)",
                    index = block_for.index,
                    lower = codegen_expr(block_for.lower.clone(), &self.dependencies)?,
                    upper = codegen_expr(block_for.upper.clone(), &self.dependencies)?
                )?;

                let child = &block_for.child;

                write!(&mut out, "{{")?;

                write!(
                    &mut out,
                    "block_{}_context *child = malloc(sizeof(*child));",
                    child.id
                )?;

                write!(&mut out, "spawn(self, block_{}, child)", child.id)?;

                write!(&mut out, "}}")?;
            }

            BlockKind::If(ref block_if) => {
                children.push(*block_if.then.clone());

                write!(
                    &mut out,
                    "if ({})",
                    codegen_cond(block_if.cond.clone(), &self.dependencies)?
                )?;

                write!(&mut out, "{{")?;

                write!(
                    &mut out,
                    "block_{}_context *child = malloc(sizeof(*child));",
                    block_if.then.id,
                )?;

                write!(&mut out, "spawn(self, block_{}, child);", block_if.then.id)?;

                write!(&mut out, "}}")?;

                if let Some(ref block_else) = block_if.else_ {
                    children.push(*block_else.clone());

                    write!(&mut out, "else {{")?;

                    write!(
                        &mut out,
                        "block_{}_context *child = malloc(sizeof(*child));",
                        block_else.id
                    )?;

                    write!(&mut out, "spawn(self, block_{}, child);", block_else.id)?;

                    write!(&mut out, "}}")?;
                }
            }

            BlockKind::Call(ref block_call) => {
                let call = &block_call.call;

                write!(&mut out, "{}(", call.ident.ident)?;

                for (i, arg) in call.args.iter().enumerate() {
                    write!(
                        &mut out,
                        "{}",
                        codegen_expr(arg.clone(), &self.dependencies)?
                    )?;

                    if i < call.args.len() - 1 {
                        write!(&mut out, ",")?;
                    }
                }

                write!(&mut out, ");")?;
            }
        }

        write!(&mut out, "return EXIT;")?;

        write!(&mut out, "}}")?;

        for child in children {
            write!(&mut out, "{}", child.code()?)?;
        }

        Ok(out)
    }
}

impl<'a> BlockTranslator<'a> {
    fn new(translator: &'a Translator) -> Self {
        Self {
            translator,
            types: HashMap::new(),
        }
    }

    fn fold_sub(&mut self, item: &ast::ItemSub) -> anyhow::Result<Block> {
        let Some(sub) = self.translator.subs.get(&item.signature.ident.ident) else {
            bail!("could not find sub {}", item.signature.ident.ident);
        };

        for param in &sub.params {
            self.types.insert(param.name.clone(), param.ty.clone());
        }

        self.fold_block(&item.block)
    }

    fn fold_block(&mut self, block: &ast::Block) -> anyhow::Result<Block> {
        let mut children = Vec::new();

        for stmt in &block.stmts {
            match stmt {
                ast::Stmt::Decl(decl) => {
                    for variable in &decl.vars {
                        self.types.insert(variable.ident.clone(), Type::Value);
                    }
                }
                ast::Stmt::Call(call) => {
                    children.push(self.fold_call(call)?);
                }
                ast::Stmt::If(if_) => {
                    children.push(self.fold_if(if_)?);
                }
                ast::Stmt::For(for_) => {
                    children.push(self.fold_for(for_)?);
                }
            }
        }

        Ok(Block {
            id: self.translator.block_id_gen.get(),
            dependencies: HashMap::new(),
            kind: BlockKind::Fork(BlockFork { children }),
        })
    }

    fn fold_for(&mut self, for_: &ast::For) -> anyhow::Result<Block> {
        let mut dependencies = HashMap::new();

        for variable in merge_sets(expr_variables(&for_.lower), expr_variables(&for_.upper)) {
            let Some(ty) = self.types.get(&variable) else {
                bail!("could not find type for {variable}");
            };

            dependencies.insert(variable, ty.clone());
        }

        let mut forked = self.clone();
        forked.types.insert(for_.index.ident.clone(), Type::Int);

        let child = forked.fold_block(&for_.block)?;

        Ok(Block {
            id: self.translator.block_id_gen.get(),
            dependencies,
            kind: BlockKind::For(BlockFor {
                index: for_.index.ident.clone(),
                lower: for_.lower.clone(),
                upper: for_.upper.clone(),
                child: Box::new(child),
            }),
        })
    }

    fn fold_if(&mut self, if_: &ast::If) -> anyhow::Result<Block> {
        let mut dependencies = HashMap::new();

        for variable in condition_variables(&if_.cond) {
            let Some(ty) = self.types.get(&variable) else {
                bail!("could not find type for {variable}");
            };

            dependencies.insert(variable.clone(), ty.clone());
        }

        let then_child = self.clone().fold_block(&if_.then)?;

        let else_child = if let Some(ref block) = if_.else_ {
            Some(self.clone().fold_block(block)?)
        } else {
            None
        };

        Ok(Block {
            id: self.translator.block_id_gen.get(),
            dependencies,
            kind: BlockKind::If(BlockIf {
                cond: if_.cond.clone(),
                then: Box::new(then_child),
                else_: else_child.map(|b| Box::new(b)),
            }),
        })
    }

    fn fold_call(&mut self, call: &ast::Call) -> anyhow::Result<Block> {
        let mut dependencies = HashMap::new();

        for arg in &call.args {
            for variable in expr_variables(arg) {
                let Some(ty) = self.types.get(&variable) else {
                    bail!("could not find type for {variable}");
                };

                dependencies.insert(variable.clone(), ty.clone());
            }
        }

        Ok(Block {
            id: self.translator.block_id_gen.get(),
            dependencies,
            kind: BlockKind::Call(BlockCall { call: call.clone() }),
        })
    }
}

fn merge_sets<T: Hash + Eq>(first: HashSet<T>, second: HashSet<T>) -> HashSet<T> {
    let mut set = HashSet::new();

    for elem in first {
        set.insert(elem);
    }

    for elem in second {
        set.insert(elem);
    }

    set
}

fn expr_variables(expr: &ast::Expr) -> HashSet<String> {
    let merge = |lhs, rhs| merge_sets(expr_variables(lhs), expr_variables(rhs));

    match expr {
        ast::Expr::Number(number) => HashSet::new(),
        ast::Expr::Ident(ident) => {
            let mut set = HashSet::new();
            set.insert(ident.ident.clone());
            set
        }

        ast::Expr::Neg(expr) => expr_variables(expr),
        ast::Expr::Add(lhs, rhs) => merge(lhs, rhs),
        ast::Expr::Sub(lhs, rhs) => merge(lhs, rhs),
        ast::Expr::Mul(lhs, rhs) => merge(lhs, rhs),
        ast::Expr::Div(lhs, rhs) => merge(lhs, rhs),
    }
}

fn condition_variables(cond: &ast::Condition) -> HashSet<String> {
    let merge_cond = |lhs, rhs| merge_sets(condition_variables(lhs), condition_variables(rhs));
    let merge_expr = |lhs, rhs| merge_sets(expr_variables(lhs), expr_variables(rhs));

    match cond {
        ast::Condition::Not(c) => condition_variables(c),
        ast::Condition::And(lhs, rhs) => merge_cond(lhs, rhs),
        ast::Condition::Or(lhs, rhs) => merge_cond(lhs, rhs),
        ast::Condition::Relation(relation) => match relation {
            ast::Relation::Equal(lhs, rhs) => merge_expr(lhs, rhs),
            ast::Relation::NotEqual(lhs, rhs) => merge_expr(lhs, rhs),
            ast::Relation::Less(rhs, lhs) => merge_expr(lhs, rhs),
            ast::Relation::LessOrEqual(lhs, rhs) => merge_expr(lhs, rhs),
            ast::Relation::Greater(lhs, rhs) => merge_expr(lhs, rhs),
            ast::Relation::GreaterOrEqual(lhs, rhs) => merge_expr(lhs, rhs),
        },
    }
}

fn codegen_expr(expr: Expr, types: &HashMap<String, Type>) -> anyhow::Result<Code> {
    let binop = |lhs, rhs, sign| -> anyhow::Result<Code> {
        Ok(format!(
            "({}{}{})",
            codegen_expr(lhs, types)?,
            sign,
            codegen_expr(rhs, types)?
        ))
    };

    let code = match expr {
        Expr::Number(number) => match number {
            ast::Number::Integer(int) => format!("{int}"),
            ast::Number::Real(float) => format!("{float}"),
        },
        Expr::Ident(ident) => {
            let Some(ty) = types.get(&ident.ident) else {
                bail!("could not find type for {}", ident.ident);
            };

            let name = &ident.ident;

            match ty {
                Type::Int => format!("((int){name})"),
                Type::Long => format!("((long){name})"),
                Type::Float => format!("((float){name})"),
                Type::Double => format!("((double){name})"),
                Type::Value => {
                    format!("df_int(&context->{name})")

                    // TODO: other types
                }

                Type::Name => {
                    format!("df_int(&context->{name})")

                    // TODO: other types
                }
            }
        }
        Expr::Neg(expr) => format!("-{}", codegen_expr(*expr, types)?),
        Expr::Add(lhs, rhs) => binop(*lhs, *rhs, "+")?,
        Expr::Sub(lhs, rhs) => binop(*lhs, *rhs, "-")?,
        Expr::Mul(lhs, rhs) => binop(*lhs, *rhs, "*")?,
        Expr::Div(lhs, rhs) => binop(*lhs, *rhs, "/")?,
    };

    Ok(code)
}

fn codegen_cond(cond: Condition, types: &HashMap<String, Type>) -> anyhow::Result<Code> {
    let code = match cond {
        Condition::Not(cond) => format!("!{}", codegen_cond(*cond, types)?),
        Condition::And(lhs, rhs) => {
            format!(
                "({}&&{})",
                codegen_cond(*lhs, types)?,
                codegen_cond(*rhs, types)?
            )
        }
        Condition::Or(lhs, rhs) => {
            format!(
                "({}||{})",
                codegen_cond(*lhs, types)?,
                codegen_cond(*rhs, types)?
            )
        }
        Condition::Relation(relation) => codegen_relation(relation, types)?,
    };

    Ok(code)
}

fn codegen_relation(relation: Relation, types: &HashMap<String, Type>) -> anyhow::Result<Code> {
    let binop = |lhs, rhs, sign| -> anyhow::Result<Code> {
        Ok(format!(
            "{}{sign}{}",
            codegen_expr(lhs, types)?,
            codegen_expr(rhs, types)?
        ))
    };

    match relation {
        Relation::Equal(lhs, rhs) => binop(lhs, rhs, "=="),
        Relation::NotEqual(lhs, rhs) => binop(lhs, rhs, "!="),
        Relation::Less(lhs, rhs) => binop(lhs, rhs, "<"),
        Relation::LessOrEqual(lhs, rhs) => binop(lhs, rhs, "<="),
        Relation::Greater(lhs, rhs) => binop(lhs, rhs, ">"),
        Relation::GreaterOrEqual(lhs, rhs) => binop(lhs, rhs, ">="),
    }
}
