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
    pub alias: Option<Ident>,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub ident: Ident,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct ItemSub {
    pub signature: Signature,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Decl(Decl),
    Call(Call),
    If(If),
    For(For),
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Condition,
    pub then: Block,
    pub else_: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub vars: Vec<Ident>,
}

#[derive(Debug, Clone)]
pub struct For {
    pub index: Ident,
    pub lower: Expr,
    pub upper: Expr,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub ident: Ident,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub ident: Option<Ident>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Long,
    Float,
    Double,
    Value,
    Name,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    Ident(Ident),
    Neg(Box<Self>),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
}

#[derive(Debug, Clone)]
pub enum Condition {
    Not(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Relation(Relation),
}

#[derive(Debug, Clone)]
pub enum Relation {
    Equal(Expr, Expr),
    NotEqual(Expr, Expr),
    Less(Expr, Expr),
    LessOrEqual(Expr, Expr),
    Greater(Expr, Expr),
    GreaterOrEqual(Expr, Expr),
}

#[derive(Debug, Clone)]
pub enum Number {
    Integer(i64),
    Real(f64),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub ident: String,
}
