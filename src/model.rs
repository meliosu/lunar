use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ops::library_call::LibraryCallOperation;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ComputationalModel {
    pub variables: HashMap<String, Variable>,
    pub operations: HashMap<String, Operation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Variable {
    #[serde(rename = "type")]
    pub r#type: Type,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Integer,
    Float,
    String,
    Slice,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Operation {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,

    #[serde(flatten)]
    pub r#type: OperationType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    LibraryCall(LibraryCallOperation),
}
