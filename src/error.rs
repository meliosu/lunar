use chumsky::error::Simple;
use thiserror::Error;

use std::ops::Range;

use crate::lexer::Token;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("lexer")]
    Lexer(#[from] LexerError),

    #[error("parser")]
    Parser(#[from] ParserError),

    #[error("translator")]
    Translator(#[from] TranslatorError),
}

#[derive(Error, Debug)]
#[error("lexer: at {span:?}")]
pub struct LexerError {
    pub span: Range<usize>,
}

#[derive(Error, Debug)]
#[error("at ?")]
pub struct ParserError {
    pub errors: Vec<Simple<Token>>,
}

#[derive(Error, Debug)]
#[error("?")]
pub struct TranslatorError {
    pub errors: Vec<anyhow::Error>,
}
