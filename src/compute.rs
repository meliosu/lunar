use crate::{
    compile::CompilationContext,
    execute::ExecutionContext,
    model::{ComputationalModel, OperationType},
    traits::{CompiledOperation, Operation},
    value::Value,
};

pub struct Runner {
    compilation_ctx: CompilationContext,
    execution_ctx: ExecutionContext,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            compilation_ctx: CompilationContext::new(),
            execution_ctx: ExecutionContext::new(),
        }
    }

    pub fn run(
        &mut self,
        model: ComputationalModel,
        inputs: Vec<(String, Value)>,
        outputs: Vec<String>,
    ) -> anyhow::Result<Vec<(String, Value)>> {
        for (name, variable) in model.variables {
            self.compilation_ctx.types.insert(name, variable.r#type);
        }

        let mut operations: Vec<Box<dyn CompiledOperation>> = Vec::new();

        for (_name, operation) in &model.operations {
            match operation.r#type {
                OperationType::LibraryCall(ref library_call_operation) => {
                    self.compilation_ctx
                        .open_library(&library_call_operation.library)?;
                }
            }
        }

        for (_name, operation) in model.operations {
            match operation.r#type {
                OperationType::LibraryCall(library_call) => {
                    let compiled = library_call.compile(
                        operation.inputs,
                        operation.outputs,
                        &self.compilation_ctx,
                    )?;

                    operations.push(Box::new(compiled));
                }
            }
        }

        for (name, value) in inputs {
            self.execution_ctx.store.insert(name, value);
        }

        while !outputs.iter().all(|o| self.execution_ctx.store.contains(o)) {
            let Some(pos) = operations
                .iter()
                .position(|op| op.ready(&self.execution_ctx))
            else {
                break;
            };

            let op = operations.remove(pos);

            op.execute(&self.execution_ctx)?;
        }

        let mut results = Vec::new();

        for output in outputs {
            if let Some(result) = self.execution_ctx.store.get(&output) {
                results.push((output, result));
            }
        }

        Ok(results)
    }
}
