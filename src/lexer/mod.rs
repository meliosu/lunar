use anyhow::anyhow;
use logos::Logos;

pub fn lex(source: &str) -> anyhow::Result<Vec<Token>> {
    Token::lexer(source).try_fold(Vec::new(), |mut acc, token| {
        acc.push(token.map_err(|e| anyhow!("{e}"))?);
        Ok(acc)
    })
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(error = String)]
pub enum Token {
    #[token("{")]
    Lbrace,

    #[token("}")]
    Rbrace,

    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token("..")]
    Dots,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token(";")]
    Semi,

    #[token(",")]
    Comma,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    #[token("import")]
    KwImport,

    #[token("as")]
    KwAs,

    #[token("sub")]
    KwSub,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("else")]
    KwElse,

    #[token("in")]
    KwIn,

    #[token("df")]
    KwDf,

    #[token("int")]
    TyInt,

    #[token("long")]
    TyLong,

    #[token("float")]
    TyFloat,

    #[token("double")]
    TyDouble,

    #[token("value")]
    TyValue,

    #[token("name")]
    TyName,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Ident(String),

    #[regex(r"0|[1-9][0-9]*", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    #[regex(r"(0|[1-9][0-9]*)[.][0-9]*", |lex| lex.slice().parse::<f64>().unwrap())]
    Real(f64),
}
