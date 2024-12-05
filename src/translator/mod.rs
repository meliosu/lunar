#![allow(unused)]

use imp::Translator;

use crate::ast;
use crate::error::TranslatorError;

pub fn translate(input: ast::File) -> Result<String, TranslatorError> {
    Ok(Translator::new().translate(input))
}

mod imp;
