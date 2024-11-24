use crate::{compile::CompilationContext, execute::ExecutionContext};

pub trait Operation<'a> {
    type Compiled;
    type Err;

    fn compile(
        &self,
        inputs: Vec<String>,
        outputs: Vec<String>,
        ctx: &'a CompilationContext,
    ) -> Result<Self::Compiled, Self::Err>;
}

pub trait CompiledOperation {
    fn ready(&self, ctx: &ExecutionContext) -> bool;
    fn execute(&self, ctx: &ExecutionContext) -> anyhow::Result<()>;
}
