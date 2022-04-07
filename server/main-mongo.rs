//! Example code for using MongoDB with Actix.

mod model;
#[cfg(test)]
mod test;

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{get, post, http, web, App, HttpResponse, HttpServer};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use model::User;
use model::Todo;
use futures::stream::{StreamExt};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {}", username))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/todos")]
async fn get_todos(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Todo> = client.database(DB_NAME).collection("todos");
    // let todos = vec![
    //     Todo {
    //         id : 1,
    //         description : "READ".to_string(),
    //         completed : false,
    //         editing : false
    //         },
    //     Todo {
    //         id : 2,
    //         description : "COOK".to_string(),
    //         completed : false,
    //         editing : false
    //         },
    //     Todo {
    //         id : 3,
    //         description : "CODING".to_string(),
    //         completed : false,
    //         editing : false
    //         },
    // ];
    // let mut res = HashMap::new();
    // res.insert(String::from("todos"), todos);
    match collection
        .find(None, None)
        .await
    {
        // To-be-fixed
        Ok(mut cursor) => {
            let mut todos = Vec::new();
            while let Some(doc) = cursor.next().await {
                if let Ok(todo) = doc {
                    todos.push(todo);
                }
            }
            let mut res = HashMap::new();
            res.insert(String::from("todos"), todos);

            HttpResponse::Ok().json(res)
        },
        // Ok(None) => {
        //     HttpResponse::NotFound().body(format!("No record found"))
        // }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[post("/api/todos")]
async fn set_todos(client: web::Data<Client>, data: web::Json<HashMap<String, Vec::<Todo>>>) -> HttpResponse {
    println!("{:?}", data);
    let todos = data.get("todos").unwrap();
    let collection: Collection<Todo> = client.database(DB_NAME).collection("todos");
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
                // collection.update_one(doc! { "id": todo.id }, todo, None);
                // HttpResponse::Ok().json(user)
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
                return HttpResponse::InternalServerError().body(err.to_string());
            },
        }

    }

    HttpResponse::Ok().body(format!("Ok"))
}

/// Creates an index on the "username" field to force the values to be unique.
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    println!("Server is connecting to {}", uri);
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

    println!("Server is running at http://127.0.0.1:8080");
    HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("http://127.0.0.1:3000")
            //   .allowed_origin_fn(|origin, _req_head| {
            //       origin.as_bytes().ends_with(b".rust-lang.org")
            //   })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(cors)
            .service(add_user)
            .service(get_user)
            .service(get_todos)
            .service(set_todos)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
