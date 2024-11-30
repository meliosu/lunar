use chumsky::{error::Simple, Parser};

use crate::lexer::Token;

pub(super) trait Parse<O>: Parser<Token, O, Error = Simple<Token>> {}
impl<O, T: Parser<Token, O, Error = Simple<Token>>> Parse<O> for T {}
