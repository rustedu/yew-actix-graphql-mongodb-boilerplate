use serde::{Deserialize, Serialize};
use async_graphql::*;


pub use crate::state::{Entry as TodoInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TodoInfoWrapper {
    pub todo: TodoInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[serde(rename_all = "camelCase")]
pub struct TodoListInfo {
    pub todos: Vec<TodoInfo>,
    // pub todos_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TodoCreateUpdateInfo {
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoCreateUpdateInfoWrapper {
    pub todo: TodoCreateUpdateInfo,
}

