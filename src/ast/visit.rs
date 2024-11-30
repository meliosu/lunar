use super::*;

pub trait Visit: Sized {
    fn visit_file(&mut self, file: &File) {
        visit_file(self, file);
    }

    fn visit_item(&mut self, item: &Item) {
        visit_item(self, item);
    }

    fn visit_import(&mut self, import: &ItemImport) {
        visit_import(self, import);
    }

    fn visit_sub(&mut self, sub: &ItemSub) {
        visit_sub(self, sub);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        visit_expr(self, expr);
    }

    fn visit_expr_ident(&mut self, ident: &Ident) {
        visit_expr_ident(self, ident);
    }

    fn visit_expr_neg(&mut self, inner: &Expr) {
        visit_expr_neg(self, inner);
    }

    fn visit_expr_number(&mut self, number: &Number) {
        visit_expr_number(self, number);
    }

    fn visit_expr_add(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_expr_add(self, lhs, rhs);
    }

    fn visit_expr_sub(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_expr_sub(self, lhs, rhs);
    }

    fn visit_expr_mul(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_expr_mul(self, lhs, rhs);
    }

    fn visit_expr_div(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_expr_div(self, lhs, rhs);
    }

    fn visit_number(&mut self, number: &Number) {
        visit_number(self, number);
    }

    fn visit_number_integer(&mut self, int: &i64) {
        visit_number_integer(self, int);
    }

    fn visit_number_float(&mut self, float: &f64) {
        visit_number_float(self, float);
    }

    fn visit_ident(&mut self, ident: &Ident) {
        visit_ident(self, ident);
    }

    fn visit_relation(&mut self, relation: &Relation) {
        visit_relation(self, relation);
    }

    fn visit_relation_eq(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_relation_eq(self, lhs, rhs);
    }

    fn visit_relation_neq(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_relation_neq(self, lhs, rhs);
    }

    fn visit_relation_lt(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_relation_lt(self, lhs, rhs);
    }

    fn visit_relation_leq(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_relation_leq(self, lhs, rhs);
    }

    fn visit_relation_gt(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_relation_gt(self, lhs, rhs);
    }

    fn visit_relation_geq(&mut self, lhs: &Expr, rhs: &Expr) {
        visit_relation_geq(self, lhs, rhs);
    }

    fn visit_cond(&mut self, cond: &Condition) {
        visit_cond(self, cond);
    }

    fn visit_cond_not(&mut self, inner: &Condition) {
        visit_cond_not(self, inner);
    }

    fn visit_cond_and(&mut self, lhs: &Condition, rhs: &Condition) {
        visit_cond_and(self, lhs, rhs);
    }

    fn visit_cond_or(&mut self, lhs: &Condition, rhs: &Condition) {
        visit_cond_or(self, lhs, rhs);
    }

    fn visit_cond_relation(&mut self, relation: &Relation) {
        visit_cond_relation(self, relation);
    }

    fn visit_type(&mut self, ty: &Type) {
        visit_type(self, ty);
    }

    fn visit_type_int(&mut self) {
        visit_type_int(self);
    }

    fn visit_type_long(&mut self) {
        visit_type_long(self);
    }

    fn visit_type_float(&mut self) {
        visit_type_float(self);
    }

    fn visit_type_double(&mut self) {
        visit_type_double(self);
    }

    fn visit_type_value(&mut self) {
        visit_type_value(self);
    }

    fn visit_type_name(&mut self) {
        visit_type_name(self);
    }

    fn visit_signature(&mut self, signature: &Signature) {
        visit_signature(self, signature);
    }

    fn visit_param(&mut self, param: &Param) {
        visit_param(self, param);
    }

    fn visit_block(&mut self, block: &Block) {
        visit_block(self, block);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        visit_stmt(self, stmt);
    }

    fn visit_stmt_for(&mut self, for_: &For) {
        visit_stmt_for(self, for_);
    }

    fn visit_stmt_if(&mut self, if_: &If) {
        visit_stmt_if(self, if_);
    }

    fn visit_stmt_decl(&mut self, decl: &Decl) {
        visit_stmt_decl(self, decl);
    }

    fn visit_stmt_call(&mut self, call: &Call) {
        visit_stmt_call(self, call);
    }
}

pub fn visit_file<V>(visit: &mut V, file: &File)
where
    V: Visit,
{
    for item in &file.items {
        visit.visit_item(item);
    }
}

pub fn visit_item<V>(visit: &mut V, item: &Item)
where
    V: Visit,
{
    match item {
        Item::Import(ref import) => visit.visit_import(import),
        Item::Sub(ref sub) => visit.visit_sub(sub),
    }
}

pub fn visit_import<V>(visit: &mut V, import: &ItemImport)
where
    V: Visit,
{
    visit.visit_signature(&import.signature);

    if let Some(ref alias) = import.alias {
        visit.visit_ident(alias);
    }
}

pub fn visit_sub<V>(visit: &mut V, sub: &ItemSub)
where
    V: Visit,
{
    visit.visit_signature(&sub.signature);
    visit.visit_block(&sub.block);
}

pub fn visit_expr<V>(visit: &mut V, expr: &Expr)
where
    V: Visit,
{
    match expr {
        Expr::Number(number) => visit.visit_expr_number(number),
        Expr::Ident(ident) => visit.visit_expr_ident(ident),
        Expr::Neg(inner) => visit.visit_expr_neg(inner),
        Expr::Add(lhs, rhs) => visit.visit_expr_add(lhs, rhs),
        Expr::Sub(lhs, rhs) => visit.visit_expr_sub(lhs, rhs),
        Expr::Mul(lhs, rhs) => visit.visit_expr_mul(lhs, rhs),
        Expr::Div(lhs, rhs) => visit.visit_expr_div(lhs, rhs),
    }
}

pub fn visit_expr_ident<V>(visit: &mut V, ident: &Ident)
where
    V: Visit,
{
    visit.visit_ident(ident);
}

pub fn visit_expr_neg<V>(visit: &mut V, inner: &Expr)
where
    V: Visit,
{
    visit.visit_expr(inner);
}

pub fn visit_expr_number<V>(visit: &mut V, number: &Number)
where
    V: Visit,
{
    visit.visit_number(number);
}

pub fn visit_expr_add<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_expr_sub<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_expr_mul<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_expr_div<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_number<V>(visit: &mut V, number: &Number)
where
    V: Visit,
{
    match number {
        Number::Integer(int) => visit.visit_number_integer(int),
        Number::Real(float) => visit.visit_number_float(float),
    }
}

pub fn visit_number_integer<V>(_visit: &mut V, _int: &i64)
where
    V: Visit,
{
}

pub fn visit_number_float<V>(_visit: &mut V, _float: &f64)
where
    V: Visit,
{
}

pub fn visit_ident<V>(_visit: &mut V, _ident: &Ident)
where
    V: Visit,
{
}

pub fn visit_relation<V>(visit: &mut V, relation: &Relation)
where
    V: Visit,
{
    match relation {
        Relation::Equal(lhs, rhs) => visit.visit_relation_eq(lhs, rhs),
        Relation::NotEqual(lhs, rhs) => visit.visit_relation_neq(lhs, rhs),
        Relation::Less(lhs, rhs) => visit.visit_relation_lt(lhs, rhs),
        Relation::LessOrEqual(lhs, rhs) => visit.visit_relation_leq(lhs, rhs),
        Relation::Greater(lhs, rhs) => visit.visit_relation_gt(lhs, rhs),
        Relation::GreaterOrEqual(lhs, rhs) => visit.visit_relation_geq(lhs, rhs),
    }
}

pub fn visit_relation_eq<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_relation_neq<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_relation_lt<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_relation_leq<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_relation_gt<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_relation_geq<V>(visit: &mut V, lhs: &Expr, rhs: &Expr)
where
    V: Visit,
{
    visit.visit_expr(lhs);
    visit.visit_expr(rhs);
}

pub fn visit_cond<V>(visit: &mut V, cond: &Condition)
where
    V: Visit,
{
    match cond {
        Condition::Not(inner) => visit.visit_cond_not(inner),
        Condition::And(lhs, rhs) => visit.visit_cond_and(lhs, rhs),
        Condition::Or(lhs, rhs) => visit.visit_cond_or(lhs, rhs),
        Condition::Relation(relation) => visit.visit_cond_relation(relation),
    }
}

pub fn visit_cond_not<V>(visit: &mut V, inner: &Condition)
where
    V: Visit,
{
    visit.visit_cond(inner);
}

pub fn visit_cond_and<V>(visit: &mut V, lhs: &Condition, rhs: &Condition)
where
    V: Visit,
{
    visit.visit_cond(lhs);
    visit.visit_cond(rhs);
}

pub fn visit_cond_or<V>(visit: &mut V, lhs: &Condition, rhs: &Condition)
where
    V: Visit,
{
    visit.visit_cond(lhs);
    visit.visit_cond(rhs);
}

pub fn visit_cond_relation<V>(visit: &mut V, relation: &Relation)
where
    V: Visit,
{
    visit.visit_relation(relation);
}

pub fn visit_type<V>(visit: &mut V, ty: &Type)
where
    V: Visit,
{
    match ty {
        Type::Int => visit.visit_type_int(),
        Type::Long => visit.visit_type_long(),
        Type::Float => visit.visit_type_float(),
        Type::Double => visit.visit_type_double(),
        Type::Value => visit.visit_type_value(),
        Type::Name => visit.visit_type_name(),
    }
}

pub fn visit_type_int<V>(_visit: &mut V)
where
    V: Visit,
{
}

pub fn visit_type_long<V>(_visit: &mut V)
where
    V: Visit,
{
}

pub fn visit_type_float<V>(_visit: &mut V)
where
    V: Visit,
{
}

pub fn visit_type_double<V>(_visit: &mut V)
where
    V: Visit,
{
}

pub fn visit_type_value<V>(_visit: &mut V)
where
    V: Visit,
{
}

pub fn visit_type_name<V>(_visit: &mut V)
where
    V: Visit,
{
}

pub fn visit_signature<V>(visit: &mut V, signature: &Signature)
where
    V: Visit,
{
    visit.visit_ident(&signature.ident);

    for param in &signature.params {
        visit.visit_param(param);
    }
}

pub fn visit_param<V>(visit: &mut V, param: &Param)
where
    V: Visit,
{
    visit.visit_type(&param.ty);

    if let Some(ref ident) = param.ident {
        visit.visit_ident(ident);
    }
}

pub fn visit_block<V>(visit: &mut V, block: &Block)
where
    V: Visit,
{
    for stmt in &block.stmts {
        visit.visit_stmt(stmt);
    }
}

pub fn visit_stmt<V>(visit: &mut V, stmt: &Stmt)
where
    V: Visit,
{
    match stmt {
        Stmt::Decl(decl) => visit.visit_stmt_decl(decl),
        Stmt::Call(call) => visit.visit_stmt_call(call),
        Stmt::If(if_) => visit.visit_stmt_if(if_),
        Stmt::For(for_) => visit.visit_stmt_for(for_),
    }
}

pub fn visit_stmt_for<V>(visit: &mut V, for_: &For)
where
    V: Visit,
{
    visit.visit_ident(&for_.index);
    visit.visit_expr(&for_.lower);
    visit.visit_expr(&for_.upper);
    visit.visit_block(&for_.block);
}

pub fn visit_stmt_if<V>(visit: &mut V, if_: &If)
where
    V: Visit,
{
    visit.visit_cond(&if_.cond);
    visit.visit_block(&if_.then);

    if let Some(ref else_) = if_.else_ {
        visit.visit_block(else_);
    }
}

pub fn visit_stmt_decl<V>(visit: &mut V, decl: &Decl)
where
    V: Visit,
{
    for var in &decl.vars {
        visit.visit_ident(var);
    }
}

pub fn visit_stmt_call<V>(visit: &mut V, call: &Call)
where
    V: Visit,
{
    visit.visit_ident(&call.ident);

    for arg in &call.args {
        visit.visit_expr(arg);
    }
}
