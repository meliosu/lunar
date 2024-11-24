use anyhow::bail;

use crate::{model::ComputationalModel, value::Value};

pub fn parse_args(
    args: Vec<String>,
    model: &ComputationalModel,
) -> anyhow::Result<(Vec<(String, Value)>, Vec<String>)> {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for arg in args {
        if let Some((name, value)) = arg.split_once("=") {
            if let Some(variable) = model.variables.get(name) {
                let value = match variable.r#type {
                    crate::model::Type::Integer => Value::Integer(value.parse()?),
                    crate::model::Type::Float => Value::Float(value.parse()?),
                    crate::model::Type::String => Value::String(value.to_owned()),
                    crate::model::Type::Slice => bail!("slice can't be an input"),
                };

                inputs.push((name.to_owned(), value));
            } else {
                bail!("can't find variable {name}");
            }
        } else {
            if model.variables.contains_key(&arg) {
                outputs.push(arg);
            } else {
                bail!("can't find variable {arg}");
            }
        }
    }

    Ok((inputs, outputs))
}
