#[macro_use] extern crate rocket;

mod auth;

use rocket::response::status;
use rocket::serde::json::{Value, json};
use crate::auth::BasicAuth;

#[get("/")]
fn hello() -> Value {
    json!("Hello World")
}

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{"id" : 1 , "name" : "Jhon Doe"}, {"id" : 2, "name" : "Jane Doe"}])
}

#[get("/rustacean/<id>")]
fn get_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!([{"id" : id, "name" : "Jane Doe"}])
}

#[post("/rustacean", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!([{"id" : 1 , "name" : "Jhon Doe"}])
}

#[put("/rustacean/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id" : id , "name" : "Jhon Doe"})
}

#[delete("/rustacean/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}


#[catch(404)]
fn not_found() -> Value {
    json!({"message": "not found"})
}

#[catch(401)]
fn not_authorized() -> Value {
    json!({"message": "not authorized"})
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            hello,
            get_rustaceans,
            get_rustacean,
            update_rustacean,
            create_rustacean,
            delete_rustacean
        ])
        .register("/", catchers![not_found, not_authorized])
        .launch()
        .await;
}