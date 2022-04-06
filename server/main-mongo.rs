//! Example code for using MongoDB with Actix.

mod model;
#[cfg(test)]
mod test;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use model::User;
use model::Todo;

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

#[get("/todos")]
async fn get_todos(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Todo> = client.database(DB_NAME).collection("todos");
    let todos = vec![
        Todo {
            id : "1".to_string(),
            description : "READ".to_string(),
            completed : false,
            editing : false
            },
        Todo {
            id : "2".to_string(),
            description : "COOK".to_string(),
            completed : false,
            editing : false
            },
        Todo {
            id : "3".to_string(),
            description : "CODING".to_string(),
            completed : false,
            editing : false
            },
    ];
    match collection
        .find(None, None)
        .await
    {
        // To-be-fixed
        Ok(_) => HttpResponse::Ok().json(todos),
        // Ok(None) => {
        //     HttpResponse::NotFound().body(format!("No record found"))
        // }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(get_user)
            .service(get_todos)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
