use super::*;

pub trait Visit<'ast> {
    fn visit_file(&mut self, file: &'ast File) {
        visit_file(self, file);
    }

    fn visit_item(&mut self, item: &'ast Item) {
        visit_item(self, item);
    }

    fn visit_item_import(&mut self, import: &'ast ItemImport) {
        visit_item_import(self, import);
    }

    fn visit_item_sub(&mut self, sub: &'ast ItemSub) {
        visit_item_sub(self, sub);
    }

    fn visit_signature(&mut self, signature: &'ast Signature) {
        visit_signature(self, signature);
    }

    fn visit_param(&mut self, param: &'ast Param) {
        visit_param(self, param);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        visit_expr(self, expr);
    }

    fn visit_expr_ident(&mut self, ident: &'ast ExprIdent) {
        visit_expr_ident(self, ident);
    }

    fn visit_expr_binop(&mut self, binop: &'ast ExprBinOp) {
        visit_expr_binop(self, binop);
    }

    fn visit_expr_lit(&mut self, lit: &'ast ExprLit) {
        visit_expr_lit(self, lit);
    }

    fn visit_lit(&mut self, lit: &'ast Lit) {
        visit_lit(self, lit);
    }

    fn visit_lit_int(&mut self, integer: &'ast LitInt) {
        visit_lit_int(self, integer);
    }

    fn visit_lit_float(&mut self, float: &'ast LitFloat) {
        visit_lit_float(self, float);
    }

    fn visit_op(&mut self, op: &'ast Op) {
        visit_op(self, op);
    }

    fn visit_block(&mut self, block: &'ast Block) {
        visit_block(self, block);
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        visit_stmt(self, stmt);
    }

    fn visit_stmt_decl(&mut self, decl: &'ast StmtDecl) {
        visit_stmt_decl(self, decl);
    }

    fn visit_stmt_block(&mut self, block: &'ast StmtBlock) {
        visit_stmt_block(self, block);
    }

    fn visit_stmt_for(&mut self, r#for: &'ast StmtFor) {
        visit_stmt_for(self, r#for);
    }

    fn visit_stmt_if(&mut self, r#if: &'ast StmtIf) {
        visit_stmt_if(self, r#if);
    }

    fn visit_stmt_call(&mut self, call: &'ast StmtCall) {
        visit_stmt_call(self, call);
    }

    fn visit_ident(&mut self, ident: &'ast Ident) {
        visit_ident(self, ident);
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        visit_type(self, ty);
    }
}

pub fn visit_file<'ast, V>(visitor: &mut V, file: &'ast File)
where
    V: Visit<'ast> + ?Sized,
{
    for item in &file.items {
        visitor.visit_item(item);
    }
}

pub fn visit_item<'ast, V>(visitor: &mut V, item: &'ast Item)
where
    V: Visit<'ast> + ?Sized,
{
    match item {
        Item::Import(item_import) => visitor.visit_item_import(item_import),
        Item::Sub(item_sub) => visitor.visit_item_sub(item_sub),
    }
}

pub fn visit_item_import<'ast, V>(visitor: &mut V, import: &'ast ItemImport)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_signature(&import.signature);
}

pub fn visit_item_sub<'ast, V>(visitor: &mut V, sub: &'ast ItemSub)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_signature(&sub.signature);
    visitor.visit_block(&sub.body);
}

pub fn visit_signature<'ast, V>(visitor: &mut V, signature: &'ast Signature)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_ident(&signature.ident);

    for param in &signature.params {
        visitor.visit_param(param);
    }
}

pub fn visit_param<'ast, V>(visitor: &mut V, param: &'ast Param)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_ident(&param.name);
    visitor.visit_type(&param.ty);
}

pub fn visit_expr<'ast, V>(visitor: &mut V, expr: &'ast Expr)
where
    V: Visit<'ast> + ?Sized,
{
    match expr {
        Expr::Ident(expr_ident) => visitor.visit_expr_ident(expr_ident),
        Expr::Lit(expr_lit) => visitor.visit_expr_lit(expr_lit),
        Expr::BinOp(expr_binop) => visitor.visit_expr_binop(expr_binop),
    }
}

pub fn visit_expr_ident<'ast, V>(visitor: &mut V, ident: &'ast ExprIdent)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_ident(&ident.ident);
}

pub fn visit_expr_binop<'ast, V>(visitor: &mut V, binop: &'ast ExprBinOp)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_expr(&binop.lhs);
    visitor.visit_op(&binop.op);
    visitor.visit_expr(&binop.rhs);
}

pub fn visit_expr_lit<'ast, V>(visitor: &mut V, lit: &'ast ExprLit)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_lit(&lit.lit);
}

pub fn visit_lit<'ast, V>(visitor: &mut V, lit: &'ast Lit)
where
    V: Visit<'ast> + ?Sized,
{
    match lit {
        Lit::Int(lit_int) => visitor.visit_lit_int(lit_int),
        Lit::Float(lit_float) => visitor.visit_lit_float(lit_float),
    }
}

pub fn visit_block<'ast, V>(visitor: &mut V, block: &'ast Block)
where
    V: Visit<'ast> + ?Sized,
{
    for stmt in &block.stmts {
        visitor.visit_stmt(stmt);
    }
}

pub fn visit_stmt<'ast, V>(visitor: &mut V, stmt: &'ast Stmt)
where
    V: Visit<'ast> + ?Sized,
{
    match stmt {
        Stmt::Decl(stmt_decl) => visitor.visit_stmt_decl(stmt_decl),
        Stmt::Block(stmt_block) => visitor.visit_stmt_block(stmt_block),
        Stmt::For(stmt_for) => visitor.visit_stmt_for(stmt_for),
        Stmt::If(stmt_if) => visitor.visit_stmt_if(stmt_if),
        Stmt::Call(stmt_call) => visitor.visit_stmt_call(stmt_call),
    }
}

pub fn visit_stmt_decl<'ast, V>(visitor: &mut V, decl: &'ast StmtDecl)
where
    V: Visit<'ast> + ?Sized,
{
    for ident in &decl.dfs {
        visitor.visit_ident(ident);
    }
}

pub fn visit_stmt_block<'ast, V>(visitor: &mut V, block: &'ast StmtBlock)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_block(&block.block);
}

pub fn visit_stmt_for<'ast, V>(visitor: &mut V, r#for: &'ast StmtFor)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_ident(&r#for.index);
    visitor.visit_expr(&r#for.lower);
    visitor.visit_expr(&r#for.upper);
    visitor.visit_block(&r#for.body);
}

pub fn visit_stmt_if<'ast, V>(visitor: &mut V, r#if: &'ast StmtIf)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_expr(&r#if.cond);
    visitor.visit_block(&r#if.then);
}

pub fn visit_stmt_call<'ast, V>(visitor: &mut V, call: &'ast StmtCall)
where
    V: Visit<'ast> + ?Sized,
{
    visitor.visit_ident(&call.ident);

    for arg in &call.args {
        visitor.visit_expr(arg);
    }
}

pub fn visit_ident<'ast, V>(_visitor: &mut V, _ident: &'ast Ident)
where
    V: Visit<'ast> + ?Sized,
{
}

pub fn visit_type<'ast, V>(_visitor: &mut V, _ty: &'ast Type)
where
    V: Visit<'ast> + ?Sized,
{
}

pub fn visit_op<'ast, V>(_visitor: &mut V, _op: &'ast Op)
where
    V: Visit<'ast> + ?Sized,
{
}

pub fn visit_lit_int<'ast, V>(_visitor: &mut V, _integer: &'ast LitInt)
where
    V: Visit<'ast> + ?Sized,
{
}

pub fn visit_lit_float<'ast, V>(_visitor: &mut V, _float: &'ast LitFloat)
where
    V: Visit<'ast> + ?Sized,
{
}
