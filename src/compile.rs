use std::collections::HashMap;

use anyhow::{anyhow, bail};
use libloading::{Library, Symbol};

use crate::model::Type;

pub struct CompilationContext {
    pub libraries: HashMap<String, Library>,
    pub types: HashMap<String, Type>,
}

impl CompilationContext {
    pub fn new() -> Self {
        Self {
            libraries: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn open_library(&mut self, name: &str) -> anyhow::Result<()> {
        if self.libraries.contains_key(name) {
            return Ok(());
        }

        // SAFETY:
        // ensuring that loaded libraries do not compromise
        // Rust memory safety is user's responsibility
        unsafe {
            let library = Library::new(&name).map_err(|e| anyhow!("{e}"))?;
            self.libraries.insert(name.into(), library);
        }

        Ok(())
    }

    pub fn find_symbol<T>(&self, library: &str, symbol: &str) -> anyhow::Result<Symbol<T>> {
        let Some(library) = self.libraries.get(library) else {
            bail!("didn't find library {library}");
        };

        // SAFETY:
        // ensuring that symbol has the right signature is the
        // responsibility of the user
        unsafe {
            library
                .get(symbol.as_bytes())
                .map_err(|e| anyhow!("didn't find symbol {symbol}: {e}"))
        }
    }
}
