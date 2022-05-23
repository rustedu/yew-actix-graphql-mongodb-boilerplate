use actix_web::{
    test::{call_and_read_body, call_and_read_body_json, init_service, TestRequest},
    web::Bytes,
    http::header
};

use mongodb::Client;
use std::str;

use serde_json::{json, Value};


use super::*;

#[actix_web::test]
#[ignore = "requires MongoDB instance running"]
async fn test() {

    let schema = build_schema().await;
    let app = init_service(
        App::new()
        // .app_data(web::Data::new(client.clone()))
        .app_data(web::Data::new(schema.clone()))
        .wrap(Logger::default())
        .service(web::resource("/graphql").guard(guard::Get()).to(index_playground))
        .service(web::resource("/graphql").guard(guard::Post()).to(index))
    )
    .await;
    let user = User { first_name: "Jane149".to_string(), last_name: "Doe149".to_string(), username: "janedoe149".to_string(), email: "example149@email.com".to_string() };
    let _add_user_graphql_request = json!(
        {
            "operationName": "addUser",
            "variables": {
                "firstName": user.first_name,
                "lastName": user.last_name,
                "username": user.username,
                "email": user.email
            },
            "query": "mutation addUser($firstName: String!, $lastName: String!, $username: String!, $email: String!) {  addUser(firstName: $firstName, lastName: $lastName, username: $username, email: $email)}"
        });

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(_add_user_graphql_request)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"{\"data\":{\"addUser\":\"user added\"}}"));

    let _get_user_graphql_request = json!(
        {
            "operationName": null,
            "variables": {},
            "query": "{\n  getUser(username: \"janedoe149\") {\n    firstName\n    lastName\n    username\n    email\n  }\n}\n"
        });
    let req = TestRequest::post()
        .uri(&format!("/graphql"))
        .set_json(_get_user_graphql_request)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    let response = str::from_utf8(&response).expect("");
    let e_user: Value = serde_json::from_str(response).unwrap();
    let simple_user = &e_user["data"]["getUser"];
    assert_eq!(simple_user["firstName"], user.first_name);
    assert_eq!(simple_user["lastName"], user.last_name);
    assert_eq!(simple_user["username"], user.username);
    assert_eq!(simple_user["email"], user.email);
}
