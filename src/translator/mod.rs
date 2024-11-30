use imp::Translator;

use crate::ast;

mod imp;

type Code = String;

pub fn translate(file: ast::File) -> anyhow::Result<Code> {
    Translator::new()
        .codegen(&file)
        .map_err(|e| anyhow::anyhow!("translator: {e}"))
}
