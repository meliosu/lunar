pub mod visit;

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Import(ItemImport),
    Sub(ItemSub),
}

#[derive(Debug, Clone)]
pub struct ItemImport {
    pub signature: Signature,
}

#[derive(Debug, Clone)]
pub struct ItemSub {
    pub signature: Signature,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub ident: Ident,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(ExprIdent),
    Lit(ExprLit),
    BinOp(ExprBinOp),
}

#[derive(Debug, Clone)]
pub struct ExprBinOp {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ExprLit {
    pub lit: Lit,
}

#[derive(Debug, Clone)]
pub struct ExprIdent {
    pub ident: Ident,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Lit {
    Int(LitInt),
    Float(LitFloat),
}

#[derive(Debug, Clone)]
pub struct LitInt {
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct LitFloat {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Decl(StmtDecl),
    Block(StmtBlock),
    For(StmtFor),
    If(StmtIf),
    Call(StmtCall),
}

#[derive(Debug, Clone)]
pub struct StmtDecl {
    pub dfs: Vec<Ident>,
}

#[derive(Debug, Clone)]
pub struct StmtBlock {
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct StmtFor {
    pub index: Ident,
    pub lower: Expr,
    pub upper: Expr,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct StmtIf {
    pub cond: Expr,
    pub then: Block,
}

#[derive(Debug, Clone)]
pub struct StmtCall {
    pub ident: Ident,
    pub args: Vec<Expr>,
}
