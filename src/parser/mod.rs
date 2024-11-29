use anyhow::bail;
use chumsky::error::Cheap;
use chumsky::prelude::*;

use crate::ast::*;
use crate::lexer::Token;

pub fn parse(input: Vec<Token>) -> anyhow::Result<File> {
    match file().parse(input) {
        Ok(file) => Ok(file),
        Err(errors) => {
            let error: String = errors
                .into_iter()
                .map(|e| format!("at {:?}", e.span()))
                .collect();

            bail!("{error}");
        }
    }
}

fn file() -> impl Parser<Token, File, Error = Cheap<Token>> {
    choice((import().map(Item::Import), sub().map(Item::Sub)))
        .repeated()
        .then_ignore(end())
        .map(|items| File { items })
}

fn ident() -> impl Parser<Token, Ident, Error = Cheap<Token>> {
    select! {
        Token::Ident(ident) => Ident { ident }
    }
}

fn ty() -> impl Parser<Token, Type, Error = Cheap<Token>> {
    select! {
        Token::TyInt => Type::Int,
        Token::TyLong => Type::Long,
        Token::TyFloat => Type::Float,
        Token::TyDouble => Type::Double,
        Token::TyValue => Type::Value,
        Token::TyName => Type::Name,
    }
}

fn signature() -> impl Parser<Token, Signature, Error = Cheap<Token>> {
    ident()
        .then(
            ty().then(ident().or_not())
                .map(|(ty, ident)| Param { ty, ident })
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::Lparen), just(Token::Rparen)),
        )
        .map(|(ident, params)| Signature { ident, params })
}

fn import() -> impl Parser<Token, ItemImport, Error = Cheap<Token>> {
    just(Token::KwImport)
        .ignore_then(signature())
        .then(just(Token::KwAs).ignore_then(ident()).or_not())
        .then_ignore(just(Token::Semi))
        .map(|(signature, alias)| ItemImport { signature, alias })
}

fn sub() -> impl Parser<Token, ItemSub, Error = Cheap<Token>> {
    signature()
        .then(block())
        .map(|(signature, block)| ItemSub { signature, block })
}

fn block() -> impl Parser<Token, Block, Error = Cheap<Token>> {
    recursive(|block| {
        let for_ = just(Token::KwFor)
            .ignore_then(ident())
            .then_ignore(just(Token::KwIn))
            .then(expr().then_ignore(just(Token::Dots)).then(expr()))
            .then(block.clone())
            .map(|((index, (lower, upper)), block)| For {
                index,
                lower,
                upper,
                block,
            });

        let if_else = just(Token::KwIf)
            .ignore_then(cond())
            .then(block.clone())
            .then_ignore(just(Token::KwElse))
            .then(block.clone())
            .map(|((cond, then), else_)| If {
                cond,
                then,
                else_: Some(else_),
            });

        let if_ = just(Token::KwIf)
            .ignore_then(cond())
            .then(block)
            .map(|(cond, then)| If {
                cond,
                then,
                else_: None,
            });

        let if_ = choice((if_else, if_));

        let call = ident()
            .then(
                expr()
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::Lparen), just(Token::Rparen)),
            )
            .then_ignore(just(Token::Semi))
            .map(|(ident, args)| Call { ident, args });

        let decl = just(Token::KwDf)
            .ignore_then(ident().separated_by(just(Token::Comma)))
            .then_ignore(just(Token::Semi))
            .map(|vars| Decl { vars });

        let stmt = choice((
            for_.map(Stmt::For),
            call.map(Stmt::Call),
            decl.map(Stmt::Decl),
            if_.map(Stmt::If),
        ));

        stmt.repeated()
            .delimited_by(just(Token::Lbrace), just(Token::Rbrace))
            .map(|stmts| Block { stmts })
    })
}

fn cond() -> impl Parser<Token, Condition, Error = Cheap<Token>> + Clone {
    recursive(|cond| {
        let relation = expr()
            .then(choice((
                just(Token::Eq).to(Relation::Equal as fn(_, _) -> _),
                just(Token::Neq).to(Relation::NotEqual as fn(_, _) -> _),
                just(Token::Lt).to(Relation::Less as fn(_, _) -> _),
                just(Token::Leq).to(Relation::LessOrEqual as fn(_, _) -> _),
                just(Token::Gt).to(Relation::Greater as fn(_, _) -> _),
                just(Token::Geq).to(Relation::GreaterOrEqual as fn(_, _) -> _),
            )))
            .then(expr())
            .map(|((lhs, op), rhs)| op(lhs, rhs));

        let atom = relation
            .map(Condition::Relation as fn(_) -> _)
            .or(cond.delimited_by(just(Token::Lparen), just(Token::Rparen)));

        let unary = just(Token::Not)
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Condition::Not(Box::new(rhs)));

        let and = unary
            .clone()
            .then(
                just(Token::And)
                    .to(Condition::And as fn(_, _) -> _)
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let or = and
            .clone()
            .then(
                just(Token::Or)
                    .to(Condition::Or as fn(_, _) -> _)
                    .then(and)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        or
    })
}

fn expr() -> impl Parser<Token, Expr, Error = Cheap<Token>> + Clone {
    recursive(|expr| {
        let number = select! {
            Token::Integer(int) => Expr::Number(Number::Integer(int)),
            Token::Real(real) => Expr::Number(Number::Real(real)),
        };

        let atom = number.or(expr.delimited_by(just(Token::Lparen), just(Token::Rparen)));

        let unary = just(Token::Sub)
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                just(Token::Mul)
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(just(Token::Div).to(Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                just(Token::Add)
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(just(Token::Sub).to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    })
}
