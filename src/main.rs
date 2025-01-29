#[macro_use] extern crate rocket;

mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use rocket::response::status;
use rocket::serde::json::{Value, json, Json};
use rocket_sync_db_pools::database;
use crate::auth::BasicAuth;

use schema::rustaceans;
use models::Rustacean;
use crate::models::NewRustacean;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!("Hello World")
}

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = rustaceans::table.order(rustaceans::id.desc()).limit(1000).load::<Rustacean>(c).expect("DB Error");
        json!(rustaceans)
    }).await
}

#[get("/rustacean/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c|{
        let result = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c).expect("DB Error");
        json!(result)
    }).await
}

#[post("/rustacean", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error inserting new rustacean");
        json!(result)
    }).await
}

#[put("/rustacean/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, rustacean: Json<Rustacean>, db: DbConn) -> Value {
    db.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set(
                (rustaceans::name.eq(rustacean.name.to_owned()),
                 rustaceans::email.eq(rustacean.email.to_owned()))
            )
            .execute(c)
            .expect("Error updating rustacean");
        json!(result)
    }).await
}

#[delete("/rustacean/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean");
        status::NoContent
    }).await
}


#[catch(404)]
fn not_found() -> Value {
    json!({"message": "not found"})
}

#[catch(401)]
fn not_authorized() -> Value {
    json!({"message": "not authorized"})
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({"message": "unprocessable entity ! Check for The Body Request "})
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
        .register("/", catchers![not_found, not_authorized, unprocessable_entity])
        .attach(DbConn::fairing())
        .launch()
        .await;
}