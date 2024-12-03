#![allow(unused)]

use chumsky::prelude::*;

use crate::ast::*;
use crate::error::ParserError;
use crate::lexer::Token;

mod traits;
use traits::Parse;

pub fn parse(input: Vec<Token>) -> Result<File, crate::error::ParserError> {
    file().parse(input).map_err(|errors| ParserError { errors })
}

fn file() -> impl Parse<File> {
    item()
        .repeated()
        .map(|items| File { items })
        .then_ignore(end())
}

fn item() -> impl Parse<Item> {
    import().map(Item::Import).or(sub().map(Item::Sub))
}

fn import() -> impl Parse<ItemImport> {
    just(Token::KwImport)
        .ignore_then(signature())
        .then_ignore(just(Token::Semi))
        .map(|signature| ItemImport { signature })
}

fn sub() -> impl Parse<ItemSub> {
    just(Token::KwSub)
        .ignore_then(signature())
        .then(block())
        .map(|(signature, body)| ItemSub { signature, body })
}

fn signature() -> impl Parse<Signature> {
    ident()
        .then(
            ty().then(ident())
                .map(|(ty, name)| Param { ty, name })
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::Lparen), just(Token::Rparen)),
        )
        .map(|(ident, params)| Signature { ident, params })
}

fn block() -> impl Parse<Block> {
    recursive(|block| {
        let stmt_decl = just(Token::KwDf)
            .ignore_then(ident().separated_by(just(Token::Comma)))
            .then_ignore(just(Token::Semi))
            .map(|dfs| StmtDecl { dfs });

        let stmt_for = just(Token::KwFor)
            .ignore_then(ident())
            .then_ignore(just(Token::KwIn))
            .then(expr())
            .then_ignore(just(Token::Dots))
            .then(expr())
            .then(block.clone())
            .map(|(((index, lower), upper), body)| StmtFor {
                index,
                lower,
                upper,
                body,
            });

        let stmt_if = just(Token::KwIf)
            .ignore_then(expr())
            .then(block.clone())
            .map(|(cond, then)| StmtIf { cond, then });

        let stmt_call = ident()
            .then(
                expr()
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::Lparen), just(Token::Rparen)),
            )
            .then_ignore(just(Token::Semi))
            .map(|(ident, args)| StmtCall { ident, args });

        let stmt_block = block.clone().map(|block| StmtBlock { block });

        let stmt = choice((
            stmt_decl.map(Stmt::Decl),
            stmt_for.map(Stmt::For),
            stmt_if.map(Stmt::If),
            stmt_call.map(Stmt::Call),
            stmt_block.map(Stmt::Block),
        ));

        stmt.repeated()
            .delimited_by(just(Token::Lbrace), just(Token::Rbrace))
            .map(|stmts| Block { stmts })
    })
}

fn expr() -> impl Parse<Expr> {
    recursive(|expr| {
        let atom = ident()
            .map(|ident| Expr::Ident(ExprIdent { ident }))
            .or(lit().map(|lit| Expr::Lit(ExprLit { lit })))
            .or(expr
                .clone()
                .delimited_by(just(Token::Lparen), just(Token::Rparen)));

        let product = atom
            .clone()
            .then(
                just(Token::Mul)
                    .to(Op::Mul)
                    .or(just(Token::Div).to(Op::Div))
                    .then(atom)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                Expr::BinOp(ExprBinOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                })
            });

        let sum = product
            .clone()
            .then(
                just(Token::Add)
                    .to(Op::Add)
                    .or(just(Token::Sub).to(Op::Sub))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                Expr::BinOp(ExprBinOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                })
            });

        sum
    })
}

fn ty() -> impl Parse<Type> + Clone {
    select! {
        Token::TyInt => Type::Int,
        Token::TyFloat => Type::Float,
        Token::TyOutput => Type::Output,
        Token::TyInput => Type::Input,
    }
}

fn lit() -> impl Parse<Lit> + Clone {
    select! {
        Token::Integer(int) => Lit::Int(LitInt { value: int }),
        Token::Float(float) => Lit::Float(LitFloat { value: float }),
    }
}

fn ident() -> impl Parse<Ident> + Clone {
    select! {
        Token::Ident(ident) => Ident { name: ident },
    }
}
