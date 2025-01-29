#[macro_use] extern crate rocket;

mod auth;
mod models;
mod schema;
mod repositories;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::{Build, Rocket};
use rocket::serde::json::{Value, json, Json};
use rocket_sync_db_pools::database;
use crate::auth::BasicAuth;

use models::Rustacean;
use crate::models::NewRustacean;


use repositories::RustaceansRepository;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!("Hello World")
}

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceansRepository::find_multiple(c, 100)
            .map(|r|{json!(r)})
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/rustacean/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>>  {
    db.run(move |c|{
        RustaceansRepository::find(c, id)
            .map(|r|{json!(r)})
            .map_err(|e|
                match e {
                    diesel::result::Error::NotFound => Custom(Status::NotFound, json!(e.to_string())),
                    _ => Custom(Status::InternalServerError, json!(e.to_string())),
                }
            )
    }).await
}

#[post("/rustacean", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>>  {
    db.run(|c| {
        RustaceansRepository::save(c, new_rustacean.into_inner())
            .map(|r|{json!(r)})
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/rustacean/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, rustacean: Json<Rustacean>, db: DbConn) -> Result<Value, Custom<Value>>  {
    db.run(move |c| {
        RustaceansRepository::update(c, id, rustacean.into_inner())
            .map(|r|{json!(r)})
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/rustacean/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        if RustaceansRepository::find(c, id)
            .is_err(){
            return Err(Custom(Status::NotFound, json!("Record Not Found")));
        }

        RustaceansRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
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

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build>{
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("unable to retrieve connection").run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations Failed");
        })
        .await;
    rocket
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
        .attach(AdHoc::on_ignite("Diesel Migrations", run_db_migrations))
        .launch()
        .await;
}