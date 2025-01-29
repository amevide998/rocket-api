use diesel::{QueryResult, SqliteConnection};
use crate::models::{NewRustacean, Rustacean};
use crate::schema::rustaceans;
use diesel::prelude::*;

pub struct RustaceansRepository;

impl RustaceansRepository {
    pub fn find(c: &mut SqliteConnection , id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit : i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.order(rustaceans::id.desc()).limit(limit).load::<Rustacean>(c)
    }

    pub fn save(c: &mut SqliteConnection , new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(crate::schema::rustaceans::table)
            .values(new_rustacean)
            .execute(c).expect("Error Create New Rustaceans");
        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(crate::schema::rustaceans::table.find(id))
            .set(
                (crate::schema::rustaceans::name.eq(rustacean.name.to_owned()),
                 crate::schema::rustaceans::email.eq(rustacean.email.to_owned()))
            )
            .execute(c).expect("Error Update New Rustaceans");
        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)

    }

    fn last_inserted_id(c : &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table.select(rustaceans::id).order(rustaceans::id.desc()).first(c)
    }
}