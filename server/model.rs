use serde::{Deserialize, Serialize};
use async_graphql::*;
use actix_web::{HttpResponse, web};
use mongodb::bson::doc;
use futures::stream::TryStreamExt;
use mongodb::{Client, options::ClientOptions, Database, Collection, bson::{Document, from_document}};
use futures::StreamExt;
use std::collections::HashMap;

pub type GraphqlResult<T> = std::result::Result<T, async_graphql::Error>;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, SimpleObject, InputObject)]
pub struct Todo {
    pub id : u32,
    pub description : String,
    pub completed : bool,
    pub editing : bool,
}


pub struct DataSource {
    client: Client,
    pub db_budshome: Database,
}

pub async fn get_user_by_username(db: Database, username: &str) -> GraphqlResult<User> {
    let coll = db.collection::<Document>("users");

    let exist_document = coll.find_one(doc! {"username": username}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = from_document(user_document)?;
            Ok(user)
        } else {
            Err(Error::new("username not found").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("Error searching mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn add_user(db: Database, first_name: &str, last_name: &str, username: &str, email: &str) -> GraphqlResult<String> {
    let coll = db.collection::<Document>("users");

    // let exist_document = coll.find_one(doc! {"username": username}, None).await;
    let insert_result = coll.insert_one(doc! {"first_name": first_name, "last_name": last_name, "username": username, "email": email}, None).await;
    if let Ok(_result) = insert_result {
        let exist_document = coll.find_one(doc! {"username": username}, None).await;
        if let Ok(user_document_exist) = exist_document {
            if let Some(user_document) = user_document_exist {
                let user: User = from_document(user_document)?;
                Ok("user added".to_string())
            } else {
                Err(Error::new("username not found").extend_with(|err, eev| {
                    eev.set("details", err.message.as_str())
                }))
            }
        } else {
            Err(Error::new("Error searching mongodb")
                .extend_with(|err, eev| eev.set("details", err.message.as_str())))
        }
    } else {
        Err(Error::new("Error insert mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn _get_todos(db: Database) -> GraphqlResult<HashMap<String, Vec<Todo>>> {
    let collection: Collection<Todo> = db.collection("todos");
    match collection
        .find(None, None)
        .await
    {
        Ok(mut cursor) => {
            let mut todos = Vec::new();
            while let Some(doc) = cursor.next().await {
                if let Ok(todo) = doc {
                    todos.push(todo);
                }
            }
            let mut res = HashMap::new();
            res.insert(String::from("todos"), todos);
            Ok(res)
        },
        Err(err) => Err(Error::new(err.to_string())),
    }
}


async fn _set_todos(db: Database, data: AllTodoList) -> GraphqlResult<String> {
    println!("{:?}", data);
    let todos = data.todos;
    let collection: Collection<Todo> = db.collection("todos");

    // always drop all records before insert
    let _d = collection.drop(None).await;

    for todo in todos {
        println!("{}", todo.id);

        match collection
            .find_one(doc! { "id": todo.id }, None)
            .await
        {
            Ok(Some(_t)) => {
                let id_filter = doc! {"id": todo.id};

                let _r = collection.update_one(
                                                                id_filter,
                                                                doc! {
                                                                    "$set": {
                                                                        "completed": todo.completed,
                                                                        "editing": todo.editing,
                                                                        "description": &todo.description,
                                                                    }
                                                                },
                                                                None
                                                            )
                                                            .await;
            },
            Ok(None) => {
                let new_todo = Todo {
                    id: todo.id,
                    description: todo.description.to_string(),
                    completed: todo.completed,
                    editing: todo.editing
                };
                let _r = collection.insert_one(new_todo, None).await.expect("failed to add todo");

            }
            Err(err) => {
                return Err(Error::new(err.to_string()));
            },
        }
    }
    Ok(format!("Ok"))
}



#[allow(dead_code)]
impl DataSource {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> DataSource {
        // Parse a connection string into an options struct.
        // environment variables defined in .env file
        let mut client_options =
            ClientOptions::parse("mongodb://localhost:27017")
                .await
                .expect("Failed to parse options!");
        // Manually set an option.
        client_options.app_name =
            Some("tide-async-graphql-mongodb".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)
            .expect("Failed to initialize database!");

        // Get a handle to a database.
        let db_budshome = client.database("myApp");

        // return mongodb datasource.
        DataSource { client: client, db_budshome: db_budshome }
    }
}

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription>
{
    // get mongodb datasource. It can be added to:
    // 1. As global data for async-graphql.
    // 2. As application scope state of Tide
    // 3. Use lazy-static.rs.
    let mongo_ds = DataSource::init().await;

    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global mongodb datasource  in the schema object.
    // let mut schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription)
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .finish()
}

pub struct QueryRoot;
#[Object]
impl QueryRoot {

    async fn get_user(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GraphqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        get_user_by_username(db, &username).await
    }

    async fn get_todos(
        &self,
        ctx: &Context<'_>
    ) -> GraphqlResult<HashMap<String, Vec<Todo>>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        _get_todos(db).await
    }
}



#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, InputObject)]
pub struct AllTodoList {
    todos: Vec<Todo>,
}


pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_user(
        &self,
        ctx: &Context<'_>,
        first_name: String,
        last_name: String,
        username: String,
        email: String,
    ) -> GraphqlResult<String> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        add_user(db, &first_name, &last_name, &username, &email).await
    }

    async fn set_todos(
        &self,
        ctx: &Context<'_>,
        data: AllTodoList
    ) -> GraphqlResult<String> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        _set_todos(db, data).await
    }
}
