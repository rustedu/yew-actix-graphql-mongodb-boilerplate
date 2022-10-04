use graphql_client::{GraphQLQuery, QueryBody, Response};
use crate::utils::{request, request_delete, request_get, request_post, request_put};
use crate::types::*;
use std::fmt;
use reqwest;
use serde_json::{Value, from_str, json};
use log;
use std::collections::HashMap;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/get_todos.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetTodos;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/set_todos.graphql",
    response_derives = "Debug, Serialize, Deserialize",
    normalization = "rust"
)]
pub struct SetTodos;

impl fmt::Debug for get_todos::Variables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}


impl fmt::Debug for set_todos::Variables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

/// Get all todos
// pub async fn get_all() -> Result<TodoListInfo, Error> {
//     request_get::<TodoListInfo>(format!("/todos")).await
//     // request_get::<TodoListInfo>(format!("/todos?{}", limit(20, 0))).await
// }

pub async fn get_all() -> Result<TodoListInfo, Error> {
    let build_query: QueryBody<get_todos::Variables> = GetTodos::build_query(get_todos::Variables{});
    let response: Result<Value, _> = request_post::<QueryBody<get_todos::Variables>, Value>(format!("/graphql"), build_query).await;
    let r_data= response.unwrap();
    let raw_result = r_data["data"]["getTodos"]["todos"].as_array().unwrap().to_owned();
    log::info!("get_all r_data {:?}", &raw_result);
    let mut result: Vec<TodoInfo> = Vec::new();
    for _todo in raw_result.into_iter() {
        let r_todo: TodoInfo = serde_json::from_value(_todo).unwrap();
        result.push(r_todo);
    }
    log::info!("get_all result {:#?}", &result);
    return Ok(TodoListInfo{ todos: result});
}

// pub async fn set_all(todos: TodoListInfo) -> Result<Response<get_todos::ResponseData>, Error>  {
//     let build_query :QueryBody<get_todos::Variables> = GetTodos::build_query(get_todos::Variables{});
//     request_post::<QueryBody<get_todos::Variables>, Response<get_todos::ResponseData>>(format!("/graphql"), build_query).await
// }

/// Set all todos
pub async fn set_all(new_todos: TodoListInfo) -> Result<TodoListInfo, Error> {
    use crate::todos::set_todos::*;
    // let mut body = HashMap::new();
    // body.insert("todos".to_string(), todos);
    let mut _raw_new_todos: Vec<NewTodo> = Vec::new();
    for _todo in new_todos.todos.into_iter() {
        let __todo = NewTodo{
            id: _todo.id.to_string().parse::<i64>().unwrap(),
            description: _todo.description,
            completed: _todo.completed,
            editing: _todo.editing
        };
        _raw_new_todos.push(__todo);
    }
    let n_todos = AllTodoList {
        todos: _raw_new_todos
    };
    let variables = set_todos::Variables {
        data: n_todos
    };
    let build_query: QueryBody<set_todos::Variables> = SetTodos::build_query(variables);
    let response = request_post::<QueryBody<set_todos::Variables>, Value>(format!("/graphql"), build_query).await;

    get_all().await
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