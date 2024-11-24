use anyhow::bail;
use libffi::{
    low::CodePtr,
    middle::{arg, Cif, Type},
};
use libloading::Symbol;
use serde::{Deserialize, Serialize};

use crate::{
    model,
    traits::{CompiledOperation, Operation},
    value::Value,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryCallOperation {
    pub library: String,
    pub symbol: String,
}

pub struct CompiledLibraryCallOperation<'a> {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub symbol: Symbol<'a, unsafe extern "C" fn()>,
    pub cif: Cif,
    pub input_types: Vec<model::Type>,
    pub output_types: Vec<model::Type>,
}

impl<'a> Operation<'a> for LibraryCallOperation {
    type Compiled = CompiledLibraryCallOperation<'a>;
    type Err = anyhow::Error;

    fn compile(
        &self,
        inputs: Vec<String>,
        outputs: Vec<String>,
        ctx: &'a crate::compile::CompilationContext,
    ) -> Result<Self::Compiled, Self::Err> {
        let symbol = ctx.find_symbol(&self.library, &self.symbol)?;

        let mut args = Vec::new();
        let mut input_types = Vec::new();
        let mut output_types = Vec::new();

        for output in &outputs {
            let Some(ty) = ctx.types.get(output) else {
                bail!("couldn't find output type for {output}");
            };

            output_types.push(*ty);

            let ty = match ty {
                crate::model::Type::Integer => Type::pointer(),
                crate::model::Type::Float => Type::pointer(),
                crate::model::Type::String => unimplemented!(),
                crate::model::Type::Slice => Type::pointer(),
            };

            args.push(ty);
        }

        for input in &inputs {
            let Some(ty) = ctx.types.get(input) else {
                bail!("couldn't find input type for {input}");
            };

            input_types.push(*ty);

            let ty = match ty {
                crate::model::Type::Integer => Type::i64(),
                crate::model::Type::Float => Type::f64(),
                crate::model::Type::String => unimplemented!(),
                crate::model::Type::Slice => Type::structure([Type::pointer(), Type::usize()]),
            };

            args.push(ty);
        }

        let cif = Cif::new(args, Type::i32());

        Ok(CompiledLibraryCallOperation {
            inputs,
            outputs,
            symbol,
            cif,
            input_types,
            output_types,
        })
    }
}

impl<'a> CompiledOperation for CompiledLibraryCallOperation<'a> {
    fn ready(&self, ctx: &crate::execute::ExecutionContext) -> bool {
        self.inputs.iter().all(|i| ctx.store.contains(i))
    }

    fn execute(&self, ctx: &crate::execute::ExecutionContext) -> anyhow::Result<()> {
        let mut outputs = Vec::new();

        for output_ty in &self.output_types {
            let value = match output_ty {
                model::Type::Integer => Value::Integer(0),
                model::Type::Float => Value::Float(0f64),
                model::Type::String => unimplemented!(),
                model::Type::Slice => Value::Slice(Box::default()),
            };

            outputs.push(Box::new(value));
        }

        let mut inputs = Vec::new();

        for input in &self.inputs {
            let Some(value) = ctx.store.get(input) else {
                bail!("error getting variable value for {input}");
            };

            inputs.push(value);
        }

        let mut args = Vec::new();

        let pointers: Vec<_> = outputs.iter().map(|o| o.ptr()).collect();

        for ptr in &pointers {
            args.push(arg(ptr));
        }

        for input in &inputs {
            match input {
                Value::Integer(int) => args.push(arg(int)),
                Value::Float(float) => args.push(arg(float)),
                Value::String(_string) => unimplemented!(),
                Value::Slice(slice) => args.push(arg(slice)),
            }
        }

        // TODO: safety
        let ret: i32 = unsafe {
            let ptr = CodePtr::from_fun(std::mem::transmute(*self.symbol));
            self.cif.call(ptr, &args)
        };

        if ret != 0 {
            bail!("subroutine failed with return code {ret}");
        }

        for (name, value) in self.outputs.iter().zip(outputs) {
            ctx.store.insert(name.to_string(), *value);
        }

        Ok(())
    }
}
