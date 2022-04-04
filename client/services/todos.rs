use crate::utils::{request_delete, request_get, request_post, request_put};
use crate::types::*;

/// Get all todos
pub async fn all() -> Result<TodoListInfo, Error> {
    request_get::<TodoListInfo>(format!("/todos")).await
    // request_get::<TodoListInfo>(format!("/todos?{}", limit(20, 0))).await
}

/// Get an todo
pub async fn get(id: String) -> Result<TodoInfoWrapper, Error> {
    request_get::<TodoInfoWrapper>(format!("/todos/{}", id)).await
}

/// Update an todo
pub async fn update(
    id: String,
    todo: TodoCreateUpdateInfoWrapper,
) -> Result<TodoInfoWrapper, Error> {
    request_put::<TodoCreateUpdateInfoWrapper, TodoInfoWrapper>(
        format!("/todos/{}", id),
        todo,
    )
    .await
}

/// Create an todo
pub async fn create(todo: TodoCreateUpdateInfoWrapper) -> Result<TodoInfoWrapper, Error> {
    request_post::<TodoCreateUpdateInfoWrapper, TodoInfoWrapper>(
        "/todos".to_string(),
        todo,
    )
    .await
}

/// Delete an todo
pub async fn del(id: String) -> Result<DeleteWrapper, Error> {
    request_delete::<DeleteWrapper>(format!("/todos/{}", id)).await
}