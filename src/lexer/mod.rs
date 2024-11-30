use logos::Logos;

use crate::error::LexerError;

pub fn lex(input: &str) -> Result<Vec<Token>, crate::error::LexerError> {
    Token::lexer(input)
        .spanned()
        .try_fold(Vec::new(), |mut acc, (token, span)| match token {
            Ok(token) => {
                acc.push(token);
                Ok(acc)
            }

            Err(_) => Err(LexerError { span }),
        })
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
pub enum Token {
    #[token("import")]
    KwImport,

    #[token("sub")]
    KwSub,

    #[token("for")]
    KwFor,

    #[token("in")]
    KwIn,

    #[token("if")]
    KwIf,

    #[token("df")]
    KwDf,

    #[token("int")]
    TyInt,

    #[token("float")]
    TyFloat,

    #[token("value")]
    TyValue,

    #[token("name")]
    TyName,

    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token("{")]
    Lbrace,

    #[token("}")]
    Rbrace,

    #[token(",")]
    Comma,

    #[token("..")]
    Dots,

    #[token(";")]
    Semi,

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

    #[regex(r"(0|[1-9][0-9]*)", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    #[regex(r"(0|[1-9][0-9]*)[.][0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),
}

impl std::cmp::Eq for Token {}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
