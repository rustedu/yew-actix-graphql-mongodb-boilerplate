use std::collections::HashMap;

pub mod todos;
pub mod error;

pub use todos::{
    TodoCreateUpdateInfo, TodoCreateUpdateInfoWrapper, TodoInfo, TodoInfoWrapper,
    TodoListInfo,
};

pub use error::{ Error, ErrorInfo };

pub type DeleteWrapper = HashMap<(), ()>;
